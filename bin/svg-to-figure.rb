#!/usr/bin/env ruby
# frozen_string_literal: true

# svg-to-figure.rb — bootstrap JSON figure specs from existing images/*.svg
#
# Usage:
#   bin/svg-to-figure.rb                          # process all images/*.svg
#   bin/svg-to-figure.rb images/basic-branching-3.svg
#
# Outputs figures/<name>.json for each SVG that doesn't already have one.
# Existing figures/*.json are never overwritten.
#
# Also emits tmp/extractor-report.json with per-figure residuals and TODO items
# so the review harness can order figures by difficulty.

require 'json'
require 'rexml/document'
require 'fileutils'

REPO_ROOT    = File.expand_path('..', __dir__)
FIGURES_DIR  = File.join(REPO_ROOT, 'figures')
IMAGES_DIR   = File.join(REPO_ROOT, 'images')
PALETTE_FILE = File.join(FIGURES_DIR, '_palette.json')
REPORT_FILE  = File.join(REPO_ROOT, 'tmp', 'extractor-report.json')

PALETTE = JSON.parse(File.read(PALETTE_FILE))

# ---------------------------------------------------------------------------
# Transform math — flatten nested SVG transform attributes to absolute coords
# ---------------------------------------------------------------------------

# Parse a single transform string into a 3x3 affine matrix [a,b,c,d,e,f]
# (same as SVG matrix(a b c d e f), applied as: x' = ax+cy+e, y' = bx+dy+f)
def parse_transform(str)
  return [1,0,0,1,0,0] if str.nil? || str.strip.empty?
  str = str.strip
  case str
  when /^translate\(\s*([-\d.]+)[\s,]+([-\d.]+)\s*\)/
    [1, 0, 0, 1, $1.to_f, $2.to_f]
  when /^translate\(\s*([-\d.]+)\s*\)/
    [1, 0, 0, 1, $1.to_f, 0]
  when /^scale\(\s*([-\d.]+)[\s,]+([-\d.]+)\s*\)/
    [$1.to_f, 0, 0, $2.to_f, 0, 0]
  when /^scale\(\s*([-\d.]+)\s*\)/
    s = $1.to_f; [s, 0, 0, s, 0, 0]
  when /^matrix\(\s*([-\d.]+)[\s,]+([-\d.]+)[\s,]+([-\d.]+)[\s,]+([-\d.]+)[\s,]+([-\d.]+)[\s,]+([-\d.]+)\s*\)/
    [$1, $2, $3, $4, $5, $6].map(&:to_f)
  else
    [1, 0, 0, 1, 0, 0]
  end
end

# Multiply two affine matrices
def mul_matrix(m1, m2)
  a1,b1,c1,d1,e1,f1 = m1
  a2,b2,c2,d2,e2,f2 = m2
  [
    a1*a2 + c1*b2,
    b1*a2 + d1*b2,
    a1*c2 + c1*d2,
    b1*c2 + d1*d2,
    a1*e2 + c1*f2 + e1,
    b1*e2 + d1*f2 + f1
  ]
end

# Apply matrix to a point
def apply_matrix(m, x, y)
  a,b,c,d,e,f = m
  [a*x + c*y + e, b*x + d*y + f]
end

# Walk the REXML tree and collect all rects/paths/texts with their absolute coords.
# Returns an array of hashes.
def collect_elements(node, parent_matrix = [1,0,0,1,0,0], results = [])
  node.elements.each do |el|
    local_tr = parse_transform(el.attributes['transform'])
    matrix   = mul_matrix(parent_matrix, local_tr)

    case el.name
    when 'rect'
      x = el.attributes['x'].to_f
      y = el.attributes['y'].to_f
      w = el.attributes['width'].to_f
      h = el.attributes['height'].to_f
      rx = el.attributes['rx'].to_f
      style = el.attributes['style'] || ''
      fill  = extract_fill(el, style)
      stroke = extract_stroke(el, style)

      ax, ay = apply_matrix(matrix, x, y)
      # Also transform width/height to handle scale()
      aw = w * matrix[0].abs
      ah = h * matrix[3].abs

      results << { type: :rect, x: ax, y: ay, w: aw, h: ah, rx: rx, fill: fill, stroke: stroke, matrix: matrix }

    when 'text'
      text_content = el.texts.map(&:value).join +
                     el.elements.map { |c| c.texts.map(&:value).join }.join
      tspans = []
      el.elements.each('tspan') { |ts| tspans << ts.text.to_s.strip }
      text_content = tspans.any? ? tspans.join("\n") : text_content.strip
      # text position from x/y attrs or tspan
      tx = (el.attributes['x'] || el.elements['tspan']&.attributes['x']).to_f
      ty = (el.attributes['y'] || el.elements['tspan']&.attributes['y']).to_f
      atx, aty = apply_matrix(matrix, tx, ty)
      results << { type: :text, x: atx, y: aty, content: text_content, lines: tspans }
    end

    collect_elements(el, matrix, results) if el.elements.size > 0
  end
  results
end

def extract_fill(el, style)
  if (m = style.match(/(?:^|;)fill:\s*([^;]+)/))
    m[1].strip
  elsif el.attributes['fill']
    el.attributes['fill']
  else
    nil
  end
end

def extract_stroke(el, style)
  if (m = style.match(/(?:^|;)stroke:\s*([^;]+)/))
    v = m[1].strip
    v unless v == 'none'
  elsif el.attributes['stroke']
    v = el.attributes['stroke']
    v unless v == 'none'
  else
    nil
  end
end

# ---------------------------------------------------------------------------
# Node classification — map (w, h, fill) → kind
# ---------------------------------------------------------------------------

COLOR_ALIASES = {
  '#efefe7' => 'commit', '#f44d27' => 'ref', '#00909a' => 'tree',
  '#cd9f00' => 'blob',   '#b8e986' => 'jessica', '#fcc161' => 'john',
  '#deb9ff' => 'josie',  'none' => nil, '#f0f0f0' => nil
}.freeze

# Normalise a fill value to a palette color name or nil
def classify_fill(fill_raw)
  return nil unless fill_raw
  f = fill_raw.downcase.strip
  return nil if f == 'none' || f == 'transparent'
  # Direct match
  return COLOR_ALIASES[f] if COLOR_ALIASES.key?(f)
  # Check palette colors
  PALETTE['colors'].each do |name, hex|
    return name if hex.downcase == f
  end
  nil
end

def classify_rect(r)
  fill_name = classify_fill(r[:fill])
  w = r[:w].round; h = r[:h].round
  rx = r[:rx].round

  # Try to match palette kinds by (w, h, shape)
  PALETTE['kinds'].each do |kind, spec|
    kw = spec['w'].to_i; kh = spec['h'].to_i
    next unless (kw - w).abs <= 2 && (kh - h).abs <= 2
    kfill = spec['fill']
    # Match by fill or shape
    if fill_name && (fill_name == kfill || PALETTE['colors'][kfill] == r[:fill]&.downcase)
      return kind
    end
    # Fallback: match by size + shape type
    pill = (spec['shape'] == 'pill')
    if pill && rx >= 10 && fill_name == kfill
      return kind
    end
  end

  # Heuristic fallbacks
  return 'commit' if fill_name == 'commit' || r[:fill]&.downcase == '#efefe7'
  return 'ref'    if fill_name == 'ref'    || r[:fill]&.downcase == '#f44d27'
  return 'tree'   if fill_name == 'tree'   || r[:fill]&.downcase == '#00909a'
  return 'blob'   if fill_name == 'blob'   || r[:fill]&.downcase == '#cd9f00'
  return 'jessica' if r[:fill]&.downcase == '#b8e986'
  return 'john'    if r[:fill]&.downcase == '#fcc161'
  return 'josie'   if r[:fill]&.downcase == '#deb9ff'
  return 'other'   if fill_name == 'commit' && r[:stroke]

  nil  # unclassified
end

# ---------------------------------------------------------------------------
# Grid inference — cluster x/y centers and fit a pitch
# ---------------------------------------------------------------------------

# Cluster a sorted array of values into groups within `tolerance`
def cluster(vals, tolerance = 8.0)
  return [] if vals.empty?
  groups = [[vals.first]]
  vals[1..].each do |v|
    if v - groups.last.last <= tolerance
      groups.last << v
    else
      groups << [v]
    end
  end
  groups.map { |g| g.sum / g.size.to_f }
end

# Given cluster centers, infer a grid pitch and origin
def infer_grid(centers)
  return { origin: centers.first, pitch: nil } if centers.size < 2
  diffs = centers.each_cons(2).map { |a, b| b - a }
  pitch = diffs.sum / diffs.size.to_f
  { origin: centers.first, pitch: pitch.round(1) }
end

# Snap a value to a grid and return (col_or_row, residual)
def snap_to_grid(val, origin, pitch)
  return [0, val - origin] unless pitch && pitch > 0
  col = ((val - origin) / pitch).round
  residual = val - (origin + col * pitch)
  [col, residual]
end

# ---------------------------------------------------------------------------
# Main extractor
# ---------------------------------------------------------------------------

def extract_figure(svg_path)
  name = File.basename(svg_path, '.svg')
  doc  = REXML::Document.new(File.read(svg_path))
  root = doc.root

  # Get the root viewBox
  vb = root.attributes['viewBox']
  canvas_w, canvas_h = if vb && (m = vb.match(/[\d.]+\s+[\d.]+\s+([\d.]+)\s+([\d.]+)/))
    [m[1].to_f, m[2].to_f]
  else
    [nil, nil]
  end

  elements = collect_elements(root)

  # Separate rects and texts
  rects = elements.select { |e| e[:type] == :rect }
  texts = elements.select { |e| e[:type] == :text }

  # Classify each rect to a node kind, filter out unclassified
  node_rects = rects.filter_map do |r|
    kind = classify_rect(r)
    next unless kind
    { kind: kind, x: r[:x], y: r[:y], w: r[:w], h: r[:h] }
  end

  # Match each node rect to nearby text
  nodes = node_rects.map.with_index do |nr, i|
    cx = nr[:x] + nr[:w] / 2.0
    cy = nr[:y] + nr[:h] / 2.0
    # Find text whose absolute x is nearest the node center and y within the node
    best = texts.min_by do |t|
      x_dist = (t[:x] - cx).abs
      in_box = t[:y] >= nr[:y] - 5 && t[:y] <= nr[:y] + nr[:h] + 5
      in_box ? x_dist : 1e9
    end
    label = if best && (best[:x] - cx).abs < nr[:w]
      best[:lines].size > 1 ? best[:lines] : best[:content].strip
    else
      "node#{i}"
    end
    { id: "node#{i}", kind: nr[:kind], label: label, x: nr[:x].round(2), y: nr[:y].round(2),
      w: nr[:w].round(2), h: nr[:h].round(2), _cx: cx, _cy: cy }
  end

  # Assign stable ids based on label content
  nodes.each do |n|
    lbl = Array(n[:label]).first.to_s.strip
    n[:id] = lbl.gsub(/[^a-z0-9]/i, '-').downcase.gsub(/-+/, '-').gsub(/^-|-$/, '')
    n[:id] = "node_#{n[:id]}" if n[:id].empty? || n[:id].match?(/^\d/)
  end
  # Deduplicate ids
  seen = Hash.new(0)
  nodes.each do |n|
    count = seen[n[:id]] += 1
    n[:id] = "#{n[:id]}-#{count}" if count > 1
  end

  # Grid inference
  xs = nodes.map { |n| n[:_cx] }.sort
  ys = nodes.map { |n| n[:_cy] }.sort
  x_centers = cluster(xs)
  y_centers = cluster(ys)
  x_grid = infer_grid(x_centers)
  y_grid = infer_grid(y_centers)

  # Snap nodes to grid
  max_residual = 0.0
  nodes.each do |n|
    col, rx = snap_to_grid(n[:_cx], x_grid[:origin], x_grid[:pitch])
    row, ry = snap_to_grid(n[:_cy], y_grid[:origin], y_grid[:pitch])
    n[:col] = col; n[:row] = row
    n[:dx]  = rx.round(2) unless rx.abs < 0.5
    n[:dy]  = ry.round(2) unless ry.abs < 0.5
    max_residual = [max_residual, rx.abs, ry.abs].max
  end

  # Compute grid pitch; fall back to per-node absolute coords if no pitch
  use_grid = x_grid[:pitch] && y_grid[:pitch] && nodes.size > 1
  grid_x   = x_grid[:pitch]&.round(1)
  grid_y   = y_grid[:pitch]&.round(1)

  # Build JSON spec
  json_nodes = nodes.map do |n|
    nd = { 'id' => n[:id], 'kind' => n[:kind], 'label' => n[:label] }
    if use_grid
      nd['col'] = n[:col]; nd['row'] = n[:row]
      nd['dx'] = n[:dx] if n[:dx]
      nd['dy'] = n[:dy] if n[:dy]
    else
      nd['x'] = n[:x]; nd['y'] = n[:y]
    end
    nd
  end

  spec = { '$schema' => './_schema.json', 'title' => name.gsub('-', ' ') }
  if use_grid
    spec['grid'] = { 'x' => grid_x, 'y' => grid_y } if grid_x && grid_y
  else
    spec['layout'] = 'absolute'
    spec['canvas'] = { 'w' => canvas_w&.round(2), 'h' => canvas_h&.round(2) }
  end
  spec['nodes'] = json_nodes
  spec['edges'] = []  # TODO: edge detection not yet implemented

  todos = []
  todos << "max grid residual #{max_residual.round(1)}px — review node positions" if max_residual > 3
  todos << "no grid pitch inferred — using absolute coords" unless use_grid
  todos << "edge detection not implemented — add edges manually"
  todos << "canvas size: #{canvas_w&.round}x#{canvas_h&.round}"

  { spec: spec, name: name, max_residual: max_residual.round(2), todos: todos, node_count: nodes.size }
end

# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

FileUtils.mkdir_p(FIGURES_DIR)
FileUtils.mkdir_p(File.join(REPO_ROOT, 'tmp'))

report_only = ARGV.delete('--report')
svg_args    = ARGV.reject { |a| a.start_with?('--') }

targets = if svg_args.empty?
  Dir[File.join(IMAGES_DIR, '*.svg')].sort
else
  svg_args
end
# Skip the legend/stencil sheet — it's not a figure
targets.reject! { |p| File.basename(p, '.svg') == 'symbols' }

report = []

targets.each do |svg_path|
  name = File.basename(svg_path, '.svg')
  out_path = File.join(FIGURES_DIR, "#{name}.json")

  begin
    result = extract_figure(svg_path)
    report << { name: name, nodes: result[:node_count], max_residual: result[:max_residual], todos: result[:todos] }

    if File.exist?(out_path)
      puts "  skip #{name} (already exists, residual #{result[:max_residual]}px)"
      next
    end

    next if report_only

    spec = result[:spec]
    json = JSON.pretty_generate(spec)
    File.write(out_path, json + "\n")
    flag = result[:max_residual] > 3 ? ' ⚠' : ''
    puts "  extracted #{name} (#{result[:node_count]} nodes, residual #{result[:max_residual]}px)#{flag}"
  rescue => e
    warn "  ERROR #{name}: #{e.message}"
    warn e.backtrace.first(3).join("\n")
    report << { name: name, error: e.message }
  end
end

# Write report sorted by max_residual descending (hard cases first)
report.sort_by! { |r| -(r[:max_residual] || 999) }
File.write(REPORT_FILE, JSON.pretty_generate(report))
puts "\nReport: #{REPORT_FILE}"

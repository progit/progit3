#!/usr/bin/env ruby
# frozen_string_literal: true

# render-figures.rb — renders figures/*.json to images/*.svg
#
# Usage:
#   bin/render-figures.rb                      # render all figures/*.json
#   bin/render-figures.rb figures/foo.json     # render one figure
#
# Requirements: Ruby stdlib only (json gem already in Gemfile).
# Output is deterministic: byte-identical across runs given the same input.

require 'json'
require 'fileutils'

REPO_ROOT    = File.expand_path('..', __dir__)
FIGURES_DIR  = ENV.fetch('FIGURES_DIR', File.join(REPO_ROOT, 'figures'))
IMAGES_DIR   = ENV.fetch('IMAGES_DIR',  File.join(REPO_ROOT, 'images'))
PALETTE_FILE = File.join(FIGURES_DIR, '_palette.json')

# ---------------------------------------------------------------------------
# Palette
# ---------------------------------------------------------------------------

module Palette
  DATA = JSON.parse(File.read(PALETTE_FILE))

  def self.color(name)
    return name if name.start_with?('#')
    DATA['colors'].fetch(name) { raise "Unknown color: #{name.inspect}" }
  end

  def self.kind(name)
    DATA['kinds'].fetch(name) { raise "Unknown kind: #{name.inspect}" }
  end
end

# ---------------------------------------------------------------------------
# Layout — resolves node positions
# ---------------------------------------------------------------------------

DEFAULT_GRID_X = 127
DEFAULT_GRID_Y  = 62
DEFAULT_MARGIN  = 4

module Layout
  # Returns { id => {x:, y:, w:, h:} } in absolute canvas coordinates.
  def self.resolve(spec)
    layout = spec['layout'] || 'grid'
    gx     = (spec.dig('grid', 'x') || DEFAULT_GRID_X).to_f
    gy     = (spec.dig('grid', 'y') || DEFAULT_GRID_Y).to_f
    margin = (spec['margin'] || DEFAULT_MARGIN).to_f
    nodes  = spec['nodes'] || []

    positions = {}
    nodes.each do |n|
      k = Palette.kind(n['kind'])
      w = (n['w'] || k['w']).to_f
      h = (n['h'] || k['h']).to_f

      if layout == 'absolute' || (n.key?('x') && n.key?('y'))
        x = n['x'].to_f
        y = n['y'].to_f
      else
        col = n.fetch('col') { raise "Node #{n['id'].inspect}: missing 'col' in grid layout" }.to_f
        row = n.fetch('row') { raise "Node #{n['id'].inspect}: missing 'row' in grid layout" }.to_f
        x = margin + col * gx
        y = margin + row * gy
      end

      x += (n['dx'] || 0).to_f
      y += (n['dy'] || 0).to_f

      positions[n['id']] = { x: x, y: y, w: w, h: h }
    end
    positions
  end

  # Returns [width, height] for the canvas.
  def self.canvas_size(spec, positions)
    return [spec.dig('canvas', 'w').to_f, spec.dig('canvas', 'h').to_f] if spec['canvas']

    margin = (spec['margin'] || DEFAULT_MARGIN).to_f
    max_x  = positions.values.map { |p| p[:x] + p[:w] }.max || 0
    max_y  = positions.values.map { |p| p[:y] + p[:h] }.max || 0

    # Also account for banners, rules, labels, groups
    [max_x + margin, max_y + margin]
  end
end

# ---------------------------------------------------------------------------
# SVG helpers
# ---------------------------------------------------------------------------

module SVG
  FONT_FAMILY  = "'JetBrains Mono', 'Source Code Pro', monospace"
  FONT_SIZE    = 12
  FONT_WEIGHT  = 700
  LINE_HEIGHT  = 13.07  # matches existing SVG line-height

  def self.esc(s)
    s.gsub('&','&amp;').gsub('<','&lt;').gsub('>','&gt;').gsub('"','&quot;')
  end

  def self.round(v)
    # Round to 3 decimal places; trim trailing zeros for compactness
    s = format('%.3f', v)
    s = s.sub(/\.?0+$/, '') if s.include?('.')
    s
  end

  # Baked-arrowhead path, matching existing SVG idiom.
  # Horizontal left-pointing arrow: shaft starts at (x1,y), tip at x2 < x1
  # m{tip_x} {cy}h{shaft}v1h-{shaft}v4l-9-4.5 9-4.5z
  def self.arrow_h(from_cx, to_cx, cy)
    # Arrowhead at left (to_cx < from_cx): tip at to_cx, shaft extends right
    tip   = round(to_cx)
    shaft = round(from_cx - to_cx - 9)  # 9px for arrowhead
    "m#{tip} #{round(cy)}h#{shaft}v1h-#{shaft}v4l-9-4.5 9-4.5z"
  end

  # Vertical downward-pointing arrow: tip at (cx, to_cy)
  # m{cx} {from_cy}v-{shaft}h1v{shaft}h4l-4.5 9-4.5-9z
  def self.arrow_v_down(cx, from_cy, to_cy)
    tip   = round(to_cy)
    shaft = round(from_cy - to_cy)
    "m#{round(cx)} #{tip}v#{shaft}h1v-#{shaft}h4l-4.5-9-4.5 9z"
  end

  # Forward (downward-pointing connector from ref to commit below):
  # tip at bottom of ref box, points down
  def self.arrow_v_down_fwd(cx, from_cy, to_cy)
    # from_cy = bottom of ref, to_cy = top of commit (arrowhead at to_cy)
    shaft = round(to_cy - from_cy - 9)
    "m#{round(cx)} #{round(from_cy)}v#{shaft}h1v-#{shaft}h4l-4.5 9-4.5-9z"
  end

  def self.diagonal_arrow(x1, y1, x2, y2)
    # Simple diagonal line with a small arrowhead drawn as a path
    dx = x2 - x1; dy = y2 - y1
    len = Math.sqrt(dx*dx + dy*dy)
    ux = dx / len; uy = dy / len
    # Arrow tip at (x2,y2), head size 9
    hsize = 9.0
    wing  = 4.5
    # Two wing points perpendicular to direction
    px = -uy; py = ux  # perpendicular
    ax = x2 - hsize * ux; ay = y2 - hsize * uy
    w1x = ax + wing * px; w1y = ay + wing * py
    w2x = ax - wing * px; w2y = ay - wing * py
    shaft_end_x = x2 - hsize * ux
    shaft_end_y = y2 - hsize * uy
    "M#{round(x1)} #{round(y1)} L#{round(shaft_end_x)} #{round(shaft_end_y)} " \
      "L#{round(w1x)} #{round(w1y)} L#{round(x2)} #{round(y2)} " \
      "L#{round(w2x)} #{round(w2y)} L#{round(shaft_end_x)} #{round(shaft_end_y)} Z"
  end

  # Render a node box (pill or rect) with optional stroke.
  def self.node_box(n, pos)
    k    = Palette.kind(n['kind'])
    x    = pos[:x]; y = pos[:y]; w = pos[:w]; h = pos[:h]
    fill  = Palette.color(n['fill']  || k['fill'])
    stroke_color = n['stroke'] || k['stroke']

    attrs = %(x="#{round(x)}" y="#{round(y)}" width="#{round(w)}" height="#{round(h)}")

    if k['shape'] == 'pill'
      rx = k['rx'] || 16.5
      attrs += %( rx="#{rx}" ry="#{rx}")
    end

    stroke_attrs = stroke_color ? %( stroke="#{Palette.color(stroke_color)}" fill="none" ) : ''
    inner = %(<rect #{attrs} fill="#{fill}"#{stroke_color ? " stroke=\"#{Palette.color(stroke_color)}\"" : ''}/>)

    label = n['label']
    lines = Array(label)
    text_fill = Palette.color(n['text'] || k['text'])
    cx = round(x + w / 2.0)

    if lines.size == 1
      # 21 = 16.5 (centre) + 4.5 (cap-height offset) — matches existing SVG baseline
      baseline_y = round(y + 21)
      tspan = %(<tspan x="#{cx}" y="#{baseline_y}" fill="#{text_fill}">#{esc(lines.first)}</tspan>)
    else
      # Multi-line: vertically centre the text block.
      # total height = (n-1)*LINE_HEIGHT + cap_height. cap_height ≈ FONT_SIZE*0.7 = 8.4px
      cap_h  = FONT_SIZE * 0.7
      total_h = (lines.size - 1) * LINE_HEIGHT + cap_h
      first_y = round(y + (h + cap_h) / 2.0 - (lines.size - 1) * LINE_HEIGHT / 2.0)
      tspan = lines.each_with_index.map do |line, i|
        dy_attr = i == 0 ? "y=\"#{first_y}\"" : "dy=\"#{LINE_HEIGHT}\""
        %(<tspan x="#{cx}" #{dy_attr} fill="#{text_fill}">#{esc(line)}</tspan>)
      end.join
    end

    text_el = %(<text font-size="#{FONT_SIZE}px" font-weight="#{FONT_WEIGHT}" ) +
              %(font-family=#{FONT_FAMILY.inspect} text-anchor="middle">#{tspan}</text>)

    "<g>\n  #{inner}\n  #{text_el}\n</g>"
  end

  # Wide banner arrow with inset text.
  # Direction: horizontal only (left or right depending on from/to x).
  def self.banner(b)
    fx = b['from']['x'].to_f; fy = b['from']['y'].to_f
    tx = b['to']['x'].to_f;   ty = b['to']['y'].to_f
    thickness = (b['thickness'] || 22).to_f
    half = thickness / 2.0
    label = b['label']
    head  = 9.0  # arrowhead protrusion

    # Horizontal only for now
    going_right = tx > fx
    if going_right
      shaft_x  = fx; tip_x = tx
      d = "M#{round(shaft_x)} #{round(fy - half)}" \
          "H#{round(tip_x - head)}" \
          "V#{round(fy - half - 4)}" \
          "L#{round(tip_x)} #{round(fy)}" \
          "L#{round(tip_x - head)} #{round(fy + half + 4)}" \
          "V#{round(fy + half)}" \
          "H#{round(shaft_x)}Z"
      text_x = round((shaft_x + tip_x - head) / 2.0)
    else
      shaft_x  = tx; tip_x = fx
      d = "M#{round(shaft_x + head)} #{round(fy - half - 4)}" \
          "L#{round(shaft_x)} #{round(fy)}" \
          "L#{round(shaft_x + head)} #{round(fy + half + 4)}" \
          "V#{round(fy + half)}" \
          "H#{round(tip_x)}" \
          "V#{round(fy - half)}" \
          "H#{round(shaft_x + head)}Z"
      text_x = round((shaft_x + head + tip_x) / 2.0)
    end

    baseline_y = round(fy + FONT_SIZE * 0.4)
    text_el = %(<text x="#{text_x}" y="#{baseline_y}" ) +
              %(font-size="#{FONT_SIZE}px" font-weight="#{FONT_WEIGHT}" ) +
              %(font-family=#{FONT_FAMILY.inspect} text-anchor="middle" ) +
              %(fill="#{Palette.color('light-text')}">#{esc(label)}</text>)

    %(<g>\n  <path d="#{d}" fill="#{Palette.color('line')}"/>\n  #{text_el}\n</g>)
  end

  # Vertical rule line
  def self.rule(r)
    if r.key?('x')
      x  = round(r['x'].to_f)
      y1 = round((r['y1'] || 0).to_f)
      y2 = round((r['y2'] || 0).to_f)
      %(<line x1="#{x}" y1="#{y1}" x2="#{x}" y2="#{y2}" stroke="#{Palette.color('rule')}" stroke-linecap="square"/>)
    else
      y  = round(r['y'].to_f)
      x1 = round((r['x1'] || 0).to_f)
      x2 = round((r['x2'] || 0).to_f)
      %(<line x1="#{x1}" y1="#{y}" x2="#{x2}" y2="#{y}" stroke="#{Palette.color('rule')}" stroke-linecap="square"/>)
    end
  end

  # Free-floating text label
  def self.floating_label(l)
    lines = Array(l['text'])
    x = round(l['x'].to_f); y = round(l['y'].to_f)
    anchor = l['anchor'] || 'middle'
    fill   = l['fill'] ? Palette.color(l['fill']) : Palette.color('commit-text')
    size   = l['size'] || FONT_SIZE

    if lines.size == 1
      %(<text x="#{x}" y="#{y}" ) +
        %(font-size="#{size}px" font-weight="#{FONT_WEIGHT}" ) +
        %(font-family=#{FONT_FAMILY.inspect} text-anchor="#{anchor}" ) +
        %(fill="#{fill}">#{esc(lines.first)}</text>)
    else
      tspans = lines.each_with_index.map do |line, i|
        dy = i == 0 ? "y=\"#{y}\"" : "dy=\"#{LINE_HEIGHT}\""
        %(<tspan x="#{x}" #{dy}>#{esc(line)}</tspan>)
      end.join
      %(<text x="#{x}" font-size="#{size}px" font-weight="#{FONT_WEIGHT}" ) +
        %(font-family=#{FONT_FAMILY.inspect} text-anchor="#{anchor}" ) +
        %(fill="#{fill}">#{tspans}</text>)
    end
  end

  # Background group / swimlane box
  def self.group_box(g)
    x = round(g['x'].to_f); y = round(g['y'].to_f)
    w = round(g['w'].to_f); h = round(g['h'].to_f)
    fill   = g['fill']   ? Palette.color(g['fill'])   : 'none'
    stroke = g['stroke'] ? Palette.color(g['stroke']) : Palette.color('rule')
    inner  = %(<rect x="#{x}" y="#{y}" width="#{w}" height="#{h}" fill="#{fill}" stroke="#{stroke}"/>)

    if g['label']
      text_el = %(<text x="#{round(g['x'].to_f + g['w'].to_f / 2.0)}" y="#{round(g['y'].to_f - 4)}" ) +
                %(font-size="12px" font-weight="700" font-family=#{FONT_FAMILY.inspect} text-anchor="middle" ) +
                %(fill="#{Palette.color('commit-text')}">#{esc(g['label'])}</text>)
      "<g>\n  #{inner}\n  #{text_el}\n</g>"
    else
      inner
    end
  end
end

# ---------------------------------------------------------------------------
# Edge routing
# ---------------------------------------------------------------------------

module Edges
  def self.render(edge, positions)
    from_id = edge['from']; to_id = edge['to']
    fp = positions.fetch(from_id) { raise "Edge refers to unknown node: #{from_id.inspect}" }
    tp = positions.fetch(to_id)   { raise "Edge refers to unknown node: #{to_id.inspect}" }
    route = edge['route'] || 'straight'
    arrow = edge['arrow'] || 'back'

    from_cx = fp[:x] + fp[:w] / 2.0
    from_cy = fp[:y] + fp[:h] / 2.0
    to_cx   = tp[:x] + tp[:w] / 2.0
    to_cy   = tp[:y] + tp[:h] / 2.0

    case route
    when 'diagonal'
      # Connect edge of 'from' box to edge of 'to' box along the diagonal
      dx = to_cx - from_cx; dy = to_cy - from_cy
      len = Math.sqrt(dx*dx + dy*dy)
      ux = dx / len; uy = dy / len
      # Start from the 'from' box edge
      half_from_w = fp[:w] / 2.0; half_from_h = fp[:h] / 2.0
      t_from = [half_from_w / ux.abs, half_from_h / uy.abs].min rescue half_from_w
      x1 = from_cx + ux * t_from; y1 = from_cy + uy * t_from
      # End at the 'to' box edge
      half_to_w = tp[:w] / 2.0; half_to_h = tp[:h] / 2.0
      t_to = [half_to_w / ux.abs, half_to_h / uy.abs].min rescue half_to_w
      x2 = to_cx - ux * t_to; y2 = to_cy - uy * t_to

      d = SVG.diagonal_arrow(x1, y1, x2, y2)
      %(<path d="#{d}" fill="#{Palette.color('line')}"/>)

    when 'straight'
      # Determine primary direction: horizontal or vertical
      horiz = (to_cy - from_cy).abs < (to_cx - from_cx).abs

      if horiz
        if arrow == 'forward'
          # 'from' box right edge → 'to' box left center: forward arrow
          x1 = fp[:x] + fp[:w]
          x2 = tp[:x]
          cy = to_cy
          shaft = SVG.round(x2 - x1 - 9)
          tip   = SVG.round(x2)
          d = "m#{SVG.round(x1)} #{SVG.round(cy)}h#{shaft}v1h-#{shaft}v4l9-4.5-9-4.5z"
        else
          # back arrow: tip at 'to' side, shaft from 'from' side
          # from.right → gap → to.right+arrowhead tip
          x1 = fp[:x]   # left of from-box = shaft start going left is from right to left
          x2 = tp[:x] + tp[:w]  # right of to-box is where arrowhead tip is
          cy = to_cy
          d = SVG.arrow_h(fp[:x], tp[:x] + tp[:w], cy - 0.5)
        end
      else
        # Vertical
        if arrow == 'forward'
          # from box bottom → to box top, pointing down
          y1 = fp[:y] + fp[:h]
          y2 = tp[:y]
          d = SVG.arrow_v_down_fwd(from_cx, y1, y2)
        else
          # back: ref above commit — tip at ref bottom, pointing up from commit top
          # existing idiom: tip at top of commit pointing upward to ref
          y1 = fp[:y]          # top of from (commit)
          y2 = tp[:y] + tp[:h] # bottom of to (ref)
          shaft = SVG.round(y1 - y2 - 9)
          d = "m#{SVG.round(from_cx)} #{SVG.round(y2)}v#{shaft}h1v-#{shaft}h4l-4.5-9-4.5 9z"
        end
      end

      %(<path d="#{d}" fill="#{Palette.color('line')}"/>)

    when 'elbow'
      # Simple elbow: horizontal then vertical
      # From right edge of from-box to top/bottom of to-box
      mid_x = (fp[:x] + fp[:w] + tp[:x] + tp[:w] / 2.0) / 2.0
      # TODO: implement elbow routing; falling back to diagonal for now
      dx = to_cx - from_cx; dy = to_cy - from_cy
      x1 = fp[:x] + fp[:w]; y1 = from_cy
      x2 = tp[:x] + tp[:w] / 2.0; y2 = tp[:y]
      d = SVG.diagonal_arrow(x1, y1, x2, y2)
      %(<path d="#{d}" fill="#{Palette.color('line')}"/>)
    end
  end
end

# ---------------------------------------------------------------------------
# Top-level renderer
# ---------------------------------------------------------------------------

def render(spec)
  positions = Layout.resolve(spec)
  cw, ch    = Layout.canvas_size(spec, positions)
  nodes     = spec['nodes']  || []
  edges     = spec['edges']  || []
  banners   = spec['banners'] || []
  rules     = spec['rules']  || []
  labels    = spec['labels'] || []
  groups    = spec['groups'] || []
  title     = spec['title'] || ''

  parts = []

  # Groups (background, drawn first)
  groups.each { |g| parts << SVG.group_box(g) }

  # Rules
  rules.each  { |r| parts << SVG.rule(r) }

  # Banners
  banners.each { |b| parts << SVG.banner(b) }

  # Edges (drawn before nodes so nodes appear on top)
  edges.each do |e|
    parts << Edges.render(e, positions)
  end

  # Nodes
  nodes.each do |n|
    parts << SVG.node_box(n, positions[n['id']])
  end

  # Free-floating labels
  labels.each { |l| parts << SVG.floating_label(l) }

  vw = SVG.round(cw); vh = SVG.round(ch)
  body = parts.map { |p| p.each_line.map { |l| "  #{l}" }.join }.join("\n")

  # Deterministic, minimal SVG — no generator comment, no timestamps, no random ids
  <<~SVG
    <?xml version="1.0" encoding="UTF-8"?>
    <svg version="1.1" viewBox="0 0 #{vw} #{vh}" xmlns="http://www.w3.org/2000/svg">
      <title>#{SVG.esc(title)}</title>
    #{body}
    </svg>
  SVG
end

# ---------------------------------------------------------------------------
# CLI entry point
# ---------------------------------------------------------------------------

def main(argv)
  targets = if argv.empty?
    Dir[File.join(FIGURES_DIR, '*.json')].reject { |f| File.basename(f).start_with?('_') }.sort
  else
    argv
  end

  targets.each do |json_path|
    begin
      spec     = JSON.parse(File.read(json_path))
      svg_data = render(spec)
      base     = File.basename(json_path, '.json')
      out_path = File.join(IMAGES_DIR, "#{base}.svg")
      File.write(out_path, svg_data)
      puts "  rendered #{base}.svg"
    rescue => e
      warn "ERROR rendering #{json_path}: #{e.message}"
      warn e.backtrace.first(3).join("\n")
      exit 1
    end
  end
end

main(ARGV) if $PROGRAM_NAME == __FILE__

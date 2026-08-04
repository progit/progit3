namespace :book do

  # Variables referenced for build
  version_string = `git describe --tags --abbrev=0`.chomp
  if version_string.empty?
    version_string = '0'
  else
    versions = version_string.split('.')
    version_string = versions[0] + '.' + versions[1] + '.' + versions[2].to_i.next.to_s
  end
  date_string = Time.now.strftime('%Y-%m-%d')
  params = "--attribute revnumber='#{version_string}' --attribute revdate='#{date_string}'"
  header_hash = `git rev-parse --short HEAD`.strip

  # Check contributors list
  # This checks commit hash stored in the header of list against current HEAD
  def check_contrib
    if File.exist?('book/contributors.txt')
      current_head_hash = `git rev-parse --short HEAD`.strip
      header = `head -n 1 book/contributors.txt`.strip
      # Match regex, then coerce resulting array to string by join
      header_hash = header.scan(/[a-f0-9]{7,}/).join

      if header_hash == current_head_hash
        puts "Hash on header of contributors list (#{header_hash}) matches the current HEAD (#{current_head_hash})"
      else
        puts "Hash on header of contributors list (#{header_hash}) does not match the current HEAD (#{current_head_hash}), refreshing"
        sh "rm book/contributors.txt"
        # Reenable and invoke task again
        Rake::Task['book/contributors.txt'].reenable
        Rake::Task['book/contributors.txt'].invoke
      end
    end
  end

  desc 'build basic book formats'
  task :build => [:build_html, :build_epub, :build_fb2, :build_mobi, :build_pdf] do
    begin
        # Run check
        Rake::Task['book:check'].invoke

        # Rescue to ignore checking errors
        rescue => e
        puts e.message
        puts 'Error when checking books (ignored)'
    end
  end

  desc 'build basic book formats (for ci)'
  task :ci => [:build_html, :build_epub, :build_fb2, :build_mobi, :build_pdf] do
      # Run check, but don't ignore any errors
      Rake::Task['book:check'].invoke
  end

  desc 'generate contributors list'
  file 'book/contributors.txt' do
      puts 'Generating contributors list'
      sh "echo 'Contributors as of #{header_hash}:\n' > book/contributors.txt"
      sh "git shortlog -s HEAD | grep -v -E '(Straub|Chacon|dependabot)' | cut -f 2- | sort | column -c 120 >> book/contributors.txt"
  end

  desc 'build HTML format'
  task :build_html => 'book/contributors.txt' do
      check_contrib()

      puts 'Converting to HTML...'
      sh "bundle exec asciidoctor #{params} -a data-uri progit.asc"
      puts ' -- HTML output at progit.html'

  end

  desc 'build Epub format'
  task :build_epub => 'book/contributors.txt' do
      check_contrib()

      puts 'Converting to EPub...'
      sh "bundle exec asciidoctor-epub3 #{params} progit.asc"
      puts ' -- Epub output at progit.epub'

  end

  desc 'build FB2 format'
  task :build_fb2 => 'book/contributors.txt' do
      check_contrib()

      puts 'Converting to FB2...'
      sh "bundle exec asciidoctor-fb2 #{params} progit.asc"
      puts ' -- FB2 output at progit.fb2.zip'

  end

  desc 'build Mobi format'
  task :build_mobi => 'book/contributors.txt' do
      check_contrib()

      puts "Converting to Mobi (kf8)..."
      sh "bundle exec asciidoctor-epub3 #{params} -a ebook-format=kf8 progit.asc"
      puts " -- Mobi output at progit.mobi"
  end

  desc 'build PDF format'
  task :build_pdf => 'book/contributors.txt' do
      check_contrib()

      puts 'Converting to PDF... (this one takes a while)'
      sh "bundle exec asciidoctor-pdf #{params} -a pdf-theme=theme/pdf/progit-theme.yml -a 'pdf-fontsdir=theme/pdf/fonts;GEM_FONTS_DIR' progit.asc 2>/dev/null"
      puts ' -- PDF output at progit.pdf'
  end

  desc 'Check generated books'
  task :check => [:build_html, :build_epub] do
      puts 'Checking generated books'

      sh "htmlproofer progit.html"
      sh "epubcheck progit.epub"
  end

  desc 'Clean all generated files'
  task :clean do
    begin
        puts 'Removing generated files'

        FileList['book/contributors.txt', 'progit.html', 'progit-kf8.epub', 'progit.epub', 'progit.fb2.zip', 'progit.mobi', 'progit.pdf'].each do |file|
            rm file

            # Rescue if file not found
            rescue Errno::ENOENT => e
              begin
                  puts e.message
                  puts 'Error removing files (ignored)'
              end
        end
    end
  end

end

task :default => "book:build"

# ---------------------------------------------------------------------------
# figures: namespace — JSON source → SVG/PNG figure pipeline
# ---------------------------------------------------------------------------

namespace :figures do
  FIGURES_DIR = File.join(__dir__, 'figures')
  IMAGES_DIR  = File.join(__dir__, 'images')
  RENDERER    = File.join(__dir__, 'bin', 'render-figures.rb')

  FIGURE_SOURCES = FileList["#{FIGURES_DIR}/*.json"].exclude("#{FIGURES_DIR}/_*.json")

  desc 'Validate all figures/*.json against figures/_schema.json'
  task :validate do
    require 'json'
    schema_path = File.join(FIGURES_DIR, '_schema.json')
    unless File.exist?(schema_path)
      abort "Missing #{schema_path}"
    end
    errors = []
    FIGURE_SOURCES.each do |f|
      begin
        JSON.parse(File.read(f))
      rescue JSON::ParserError => e
        errors << "#{f}: #{e.message}"
      end
    end
    if errors.any?
      errors.each { |e| puts "  INVALID: #{e}" }
      abort "#{errors.size} invalid figure(s)"
    end
    puts "  #{FIGURE_SOURCES.size} figure(s) valid JSON"
  end

  desc 'Render figures/*.json to images/*.svg'
  task :build => :validate do
    sh "ruby #{RENDERER}"
  end

  desc 'Render a single figure by name (e.g. rake figures:render[basic-branching-1])'
  task :render, [:name] => :validate do |_t, args|
    name = args[:name] or abort "Usage: rake figures:render[name]"
    src  = File.join(FIGURES_DIR, "#{name}.json")
    abort "No such figure: #{src}" unless File.exist?(src)
    sh "ruby #{RENDERER} #{src}"
  end

  desc 'Rasterize images/*.svg (from figures/) to images/*.png at 3x via rsvg-convert'
  task :raster => :build do
    rsvg = `which rsvg-convert`.strip
    abort "rsvg-convert not found — install librsvg (brew install librsvg)" if rsvg.empty?

    # Font check: verify rsvg-convert resolves JetBrains Mono via fontconfig
    font_check = `fc-match 'JetBrains Mono' 2>/dev/null`.strip
    unless font_check.downcase.include?('jetbrainsmono')
      abort "JetBrains Mono not found by fontconfig.\n" \
            "Locally: install the font. CI: set XDG_DATA_HOME or FONTCONFIG_FILE " \
            "to expose theme/pdf/fonts/."
    end

    FIGURE_SOURCES.each do |json_path|
      base     = File.basename(json_path, '.json')
      svg_path = File.join(IMAGES_DIR, "#{base}.svg")
      png_path = File.join(IMAGES_DIR, "#{base}.png")
      next unless File.exist?(svg_path)

      # Read the viewBox to compute a 3x width
      vb = File.read(svg_path)[/viewBox="0 0 ([\d.]+) ([\d.]+)"/, 1]
      if vb
        width = (vb.to_f * 3).round
        sh "rsvg-convert -w #{width} #{svg_path} -o #{png_path}"
      else
        sh "rsvg-convert #{svg_path} -o #{png_path}"
      end
    end
  end

  desc 'Fail if committed images/ is stale relative to figures/ (run on CI after figures:build)'
  task :check => :validate do
    require 'tmpdir'
    require 'json'
    Dir.mktmpdir('figures-check') do |tmpdir|
      # Render all figures into a temp dir and compare with committed images/
      renderer_env = "FIGURES_DIR=#{FIGURES_DIR} IMAGES_DIR=#{tmpdir}"
      sh "#{renderer_env} ruby #{RENDERER}", :verbose => false do |ok, _|
        abort "Renderer failed" unless ok
      end
      stale = []
      FIGURE_SOURCES.each do |json_path|
        base      = File.basename(json_path, '.json')
        committed = File.join(IMAGES_DIR, "#{base}.svg")
        rendered  = File.join(tmpdir, "#{base}.svg")
        if File.exist?(committed) && File.exist?(rendered)
          stale << base if File.read(rendered) != File.read(committed)
        else
          stale << base
        end
      end
      if stale.any?
        stale.each { |b| puts "  STALE: #{b}.svg" }
        abort "#{stale.size} stale figure(s) — run: rake figures:build && git add images/"
      end
      puts "  All #{FIGURE_SOURCES.size} figure(s) up to date"
    end
  end

  desc 'Build a side-by-side review HTML page at tmp/figure-review.html'
  task :review => :build do
    require 'fileutils'
    FileUtils.mkdir_p File.join(__dir__, 'tmp')
    out = File.join(__dir__, 'tmp', 'figure-review.html')

    rows = FIGURE_SOURCES.sort.map do |json_path|
      base     = File.basename(json_path, '.json')
      old_png  = "file://#{IMAGES_DIR}/#{base}.png"
      new_svg  = "file://#{IMAGES_DIR}/#{base}.svg"
      master_flag = File.read(json_path).include?('"master"') ? '<b style="color:red">contains master</b>' : ''
      <<~ROW
        <tr>
          <td style="font-family:monospace;padding:4px 8px;vertical-align:top">#{base}</td>
          <td style="padding:4px">#{master_flag}</td>
          <td style="padding:4px"><img src="#{old_png}" style="max-width:600px;border:1px solid #ccc"/></td>
          <td style="padding:4px"><img src="#{new_svg}" style="max-width:600px;border:1px solid #ccc"/></td>
        </tr>
      ROW
    end.join

    File.write(out, <<~HTML)
      <!DOCTYPE html>
      <html><head><meta charset="utf-8">
      <title>Figure review</title>
      <style>body{font-family:sans-serif;font-size:13px} th{text-align:left;padding:4px 8px;background:#eee}</style>
      </head><body>
      <h1>Figure review — #{FIGURE_SOURCES.size} figures</h1>
      <p>Left: committed PNG &nbsp;|&nbsp; Right: new SVG render</p>
      <table cellspacing="0" cellpadding="0" style="border-collapse:collapse">
        <tr><th>Name</th><th>Flags</th><th>Old PNG</th><th>New SVG</th></tr>
        #{rows}
      </table>
      </body></html>
    HTML
    puts "  Review page: #{out}"
    puts "  Open with: open tmp/figure-review.html"
  end
end

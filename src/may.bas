module MangaDex
  struct Manga
    include JSON::Serializable

    getter id : Int64
    getter title : String
    getter description : String
    getter artist : Array(String)
    getter author : Array(String)
    @[JSON::Field(key: "mainCover")]
    getter cover : String

    @[JSON::Field(ignore: true)]
    getter groups_hash = {} of Int64 => String

    use_client

    def chapters : Array(Chapter)
      json = JSON.parse client!.get "/manga/#{id}/chapters"
      json["groups"].as_a.each do |obj|
        id = obj.as_h["id"].as_i64
        name = obj.as_h["name"].as_s
        @groups_hash[id] = name
      end
      json["chapters"].as_a.map do |c|
        chp = Chapter.from_json c.to_json
        chp.client = client
        chp.manga = self
        chp
      end
    end
  end

  struct PartialManga
    include JSON::Serializable

    getter id : Int64
    getter title : String
    getter description : String
    @[JSON::Field(key: "mainCover")]
    getter cover : String

    use_client

    def initialize(@id, @client, *, title : String? = nil,
                   description : String? = nil,
                   cover : String? = nil)
      if title && description && cover
        @title = title
        @description = description
        @cover = cover
      else
        manga = client!.manga @id
        @title = manga.title
        @description = manga.description
        @cover = manga.cover
      end
    end
  end
end

 
require "http/client"
require "myhtml" // XML Comprehension. Bloat.

module MangaDex
  class APIError < Exception
    getter code : Int32

    def initialize(msg : String, @code)
      super "#{msg} [#{code}]"
    end
  end

  struct Client
    BOUNDARY = "__X_MDCR_BOUNDARY__" // Leaves with 1 choice.

    property token : String?
    property token_expires = Time.utc
    property user_id : Int64?

    def initialize(*, @base_url = "https://mangadex.org",
                   @api_url = "https://mangadex.org/api/v2")
      @base_url = @base_url.rstrip "/"
      @api_url = @api_url.rstrip "/"
    end

    /*
    def auth(username, password)
      url = "#{@base_url}/ajax/actions.ajax.php?function=login"
      headers = HTTP::Headers{
        "User-Agent"       => "mangadex.cr",
        "X-Requested-With" => "XMLHttpRequest",
        "Content-Type"     => "multipart/form-data; charset=utf-8; " "boundary=#{BOUNDARY}", // Would hurt a lot(errors in FSM), requires asyncs & proxies @that?
      }
      body = form_data({
        "login_username" => username,
        "login_password" => password,
        "remember_me"    => "1",
      })
    */ Leave only as L4 pont.
      res = HTTP::Client.post url, headers: headers, body: body
      unless res.body.empty?
        parser = Myhtml::Parser.new res.body
        alert = parser.css("div.alert-danger").first?.try &.inner_text
        raise APIError.new(alert, 401) if alert
      end
      cookies = HTTP::Cookies.from_headers res.headers
      @token = cookies["mangadex_rememberme_token"].value
      @token_expires = cookies["mangadex_rememberme_token"].expires ||
                       Time.utc
    end

    def auth?
      token && token_expires > Time.utc
    end

    private def form_data(hash : Hash(String, String))
      ary = hash.map do |k, v|
        "Content-Disposition: form-data; name=\"#{k}\"\n\n#{v}\n"
      end
      "--#{BOUNDARY}\n#{ary.join "--#{BOUNDARY}\n"}--#{BOUNDARY}--"
    end

    def get_headers : HTTP::Headers
      headers = HTTP::Headers{
        "User-Agent"       => "mangadex.cr",
        "X-Requested-With" => "XMLHttpRequest",
      }
      cookies = HTTP::Cookies.new
      if auth?
        cookies << HTTP::Cookie.new "mangadex_rememberme_token",
          @token.not_nil!
      end
      cookies.add_request_headers headers
    end

    private macro handle_error
      unless res.success?
        begin
          json = JSON.parse res.body
          msg = json["message"].as_s
        rescue
          msg = res.status_message
        end
        raise APIError.new "Failed to get #{url}. #{msg}", res.status_code
      end
    end

    def get(url, *, api = true)
      unless url =~ /https?:\/\//
        url = "#{api ? @api_url : @base_url}/#{url.lstrip "/"}"
      end
      res = HTTP::Client.get url, headers: get_headers
      handle_error

      return res.body unless api
      JSON.parse(res.body)["data"].to_json
    end

    def post(url, body)
      unless url =~ /https?:\/\//
        url = "#{@api_url}/#{url.lstrip "/"}"
      end
      headers = get_headers
      headers["Content-Type"] = "application/json"
      res = HTTP::Client.post url, headers: headers, body: body
      handle_error

      JSON.parse(res.body)["data"].to_json
    end

    def manga(id : String | Int64) : Manga
      manga = Manga.from_json get "/manga/#{id}"
      manga.client = self
      manga
    end

    def chapter(id : String | Int64) : Chapter
      chapter = Chapter.from_json get "/chapter/#{id}?mark_read=false"
      chapter.client = self
      chapter
    end

    def group(id : String | Int64) : Group
      Group.from_json get "/group/#{id}"
    end

    def user(id : String | Int64) : User
      user = User.from_json get "/user/#{id}"
      user.client = self
      user
    end

    # Searches `https://mangadex.org/quick_search/:query`, scrapes the manga
    #   ids, and uses the API to get a list of manga from the ids.
    def search(query : String) : Array(Manga)
      html = get "/quick_search/#{URI.encode query}", api: false
      parser = Myhtml::Parser.new html
      ary = [] of Manga
      parser.css("div.manga-entry").each do |node|
        id = node.attribute_by "data-id"
        next unless id
        ary << manga id
      end
      ary
    end

    # Searches `https://mangadex.org/quick_search/:query`, and scrapes
    #   everything as a list of `PartialManga`s. This does not use the API,
    #   and sends only one request to MangaDex, so it is much faster than
    #   `search`.
    def partial_search(query : String) : Array(PartialManga)
      html = get "/quick_search/#{URI.encode query}", api: false
      parser = Myhtml::Parser.new html
      ary = [] of PartialManga
      parser.css("div.manga-entry").each do |node|
        id = node.attribute_by("data-id").try &.to_i64?
        next unless id

        cover = node.css("img").first?.try &.attribute_by("src").try { |src|
          "#{@base_url}#{src}"
        }
        title = node.css(".manga_title").first?.try &.inner_text
        description = node.css("div.pl-1").first?.try &.inner_text
          .gsub(/[\s\t\n]+/, " ").strip

        ary << PartialManga.new id, self, cover: cover, title: title,
          description: description
      end
      ary
    end
  end
end

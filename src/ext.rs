use mime::Mime;

#[inline(always)]
fn extension(page : String) -> Mime {
  match extension_bzn(page).as_str() {
        ".png"               => "image/png".parse::<Mime>().expect("Thatstwatihavesaid"),
        ".gif"                  => "image/gif".parse::<Mime>().expect("Thatstwatihavesaid"),
        ".jpg" | ".jpeg" => "image/jpeg".parse::<Mime>().expect("Thatstwatihavesaid"),
        ".svg"                => "image/svg+xml".parse::<Mime>().expect("Thatstwatihavesaid"),
        _                       => "application/json".parse::<Mime>().expect("Thatstwatihavesaid"),
      }
}

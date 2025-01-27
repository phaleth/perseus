mod index;

use perseus::define_app;
define_app! {
    templates: [
        index::get_template::<G>()
    ],
    error_pages: perseus::ErrorPages::new(|url, status, err, _| {
        sycamore::view! {
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }
    }),
    static_aliases: {
        "/message" => "message.txt"
    }
}

use std::path::Path;
use std::collections::BTreeMap;

use rustc_serialize::json::{self, ToJson};
use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context, Renderable};

// Handlebars helper for navigation

pub fn previous(c: &Context, _h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    debug!("[fn]: previous (handlebars helper)");

    debug!("[*]: Get data from context");
    // get value from context data
    // rc.get_path() is current json parent path, you should always use it like this
    // param is the key of value you want to display
    let chapters = c.navigate(rc.get_path(), "chapters");

    let current = c.navigate(rc.get_path(), "path")
        .to_string()
        .replace("\"", "");


    debug!("[*]: Decode chapters from JSON");
    // Decode json format
    let decoded: Vec<BTreeMap<String, String>> = match json::decode(&chapters.to_string()) {
        Ok(data) => data,
        Err(_) => return Err(RenderError::new("Could not decode the JSON data")),
    };
    let mut previous: Option<BTreeMap<String, String>> = None;


    debug!("[*]: Search for current Chapter");
    // Search for current chapter and return previous entry
    for item in decoded {

        match item.get("path") {
            Some(path) if !path.is_empty() => {
                if path == &current {

                    debug!("[*]: Found current chapter");
                    if let Some(previous) = previous {

                        debug!("[*]: Creating BTreeMap to inject in context");
                        // Create new BTreeMap to extend the context: 'title' and 'link'
                        let mut previous_chapter = BTreeMap::new();

                        // Chapter title
                        match previous.get("name") {
                            Some(n) => {
                                debug!("[*]: Inserting title: {}", n);
                                previous_chapter.insert("title".to_owned(), n.to_json())
                            },
                            None => {
                                debug!("[*]: No title found for chapter");
                                return Err(RenderError::new("No title found for chapter in JSON data"));
                            },
                        };

                        // Chapter link

                        match previous.get("path") {
                            Some(p) => {
                                // Hack for windows who tends to use `\` as separator instead of `/`
                                let path = Path::new(p).with_extension("html");
                                debug!("[*]: Inserting link: {:?}", path);

                                match path.to_str() {
                                    Some(p) => {
                                        previous_chapter.insert("link".to_owned(), p.replace("\\", "/").to_json());
                                    },
                                    None => return Err(RenderError::new("Link could not be converted to str")),
                                }
                            },
                            None => return Err(RenderError::new("No path found for chapter in JSON data")),
                        }

                        debug!("[*]: Inject in context");
                        // Inject in current context
                        let updated_context = c.extend(&previous_chapter);

                        debug!("[*]: Render template");
                        // Render template
                        match _h.template() {
                            Some(t) => {
                                try!(t.render(&updated_context, r, rc));
                            },
                            None => return Err(RenderError::new("Error with the handlebars template")),
                        }

                    }

                    break;
                } else {
                    previous = Some(item.clone());
                }
            },
            _ => continue,

        }

    }

    Ok(())
}




pub fn next(c: &Context, _h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    debug!("[fn]: next (handlebars helper)");

    debug!("[*]: Get data from context");
    // get value from context data
    // rc.get_path() is current json parent path, you should always use it like this
    // param is the key of value you want to display
    let chapters = c.navigate(rc.get_path(), "chapters");

    let current = c.navigate(rc.get_path(), "path")
        .to_string()
        .replace("\"", "");

    debug!("[*]: Decode chapters from JSON");
    // Decode json format
    let decoded: Vec<BTreeMap<String, String>> = match json::decode(&chapters.to_string()) {
        Ok(data) => data,
        Err(_) => return Err(RenderError::new("Could not decode the JSON data")),
    };
    let mut previous: Option<BTreeMap<String, String>> = None;

    debug!("[*]: Search for current Chapter");
    // Search for current chapter and return previous entry
    for item in decoded {

        match item.get("path") {

            Some(path) if !path.is_empty() => {

                if let Some(previous) = previous {

                    let previous_path = match previous.get("path") {
                        Some(p) => p,
                        None => return Err(RenderError::new("No path found for chapter in JSON data")),
                    };

                    if previous_path == &current {

                        debug!("[*]: Found current chapter");
                        debug!("[*]: Creating BTreeMap to inject in context");
                        // Create new BTreeMap to extend the context: 'title' and 'link'
                        let mut next_chapter = BTreeMap::new();

                        match item.get("name") {
                            Some(n) => {
                                debug!("[*]: Inserting title: {}", n);
                                next_chapter.insert("title".to_owned(), n.to_json());
                            },
                            None => return Err(RenderError::new("No title found for chapter in JSON data")),
                        }


                        let link = Path::new(path).with_extension("html");
                        debug!("[*]: Inserting link: {:?}", link);

                        match link.to_str() {
                            Some(l) => {
                                // Hack for windows who tends to use `\` as separator instead of `/`
                                next_chapter.insert("link".to_owned(), l.replace("\\", "/").to_json());
                            },
                            None => return Err(RenderError::new("Link could not converted to str")),
                        }

                        debug!("[*]: Inject in context");
                        // Inject in current context
                        let updated_context = c.extend(&next_chapter);

                        debug!("[*]: Render template");

                        // Render template
                        match _h.template() {
                            Some(t) => {
                                try!(t.render(&updated_context, r, rc));
                            },
                            None => return Err(RenderError::new("Error with the handlebars template")),
                        }

                        break;
                    }
                }

                previous = Some(item.clone());
            },

            _ => continue,
        }
    }
    Ok(())
}

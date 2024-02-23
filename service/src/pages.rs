use humansize::{format_size, DECIMAL};
use indexmap::IndexMap;
use maud::{html, Markup, Render, DOCTYPE};
use serde::Deserialize;
use std::{collections::HashMap, fmt::Write as _};

use crate::Stats;

struct Raw(String);

impl Render for Raw {
    fn render_to(&self, output: &mut String) {
        write!(output, "{}", self.0).unwrap();
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PolyfillData {
    polyfills: Vec<String>,
    polyfill_aliases: HashMap<String, Vec<String>>,
    // version: String,
}

fn layout(content: Markup) -> Markup {
    let stylee = Raw(include_str!("style.css").to_string());
    let scriptt = Raw(include_str!("script.js").to_string());
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta http-equiv="x-ua-compatible" content="ie=edge";
                meta name="description" content="Fastly Polyfill is a service which accepts a request for a set of browser features and returns only the polyfills that are needed by the requesting browser.";
                meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no";
                title {"Fastly Polyfill";}
                link rel="icon" href="/img/fastly-favicon.svg";
                style {(stylee);};
            }
            body {
                nav class="container-fluid" {
                    ul {
                        li {
                            a href="/" aria-label="Back home" {
                                img focusable="false" style="height: 56px;" src="/img/fastly.svg";
                            }
                        }
                        li {  "Fastly Polyfill" }
                    }
                    ul {
                        li {
                            a href="https://www.fastly.com/terms/" class="secondary" {
                                "Terms & Conditions"
                            }
                        }
                        li{
                            a href="https://www.fastly.com/privacy/" class="secondary" {
                                "Privacy Policy"
                            }
                        }
                        li{
                            a href="https://github.com/fastly/polyfill-service/" class="contrast" aria-label="GitHub repository" {
                                svg aria-hidden="true" focusable="false" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 496 512" height="16px" {
                                    path fill="currentColor" d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z";
                                }
                            }
                        }
                    }
                }
                (content)
                script defer="" {
                    (scriptt);
                }
            }
        }
    }
}

fn humanize(num: u64) -> String {
    let num3 = num.to_string();
    match num3.len() {
        1 | 2 | 3 => num3,
        4 => {
            let x = num3.chars().nth(1).unwrap();
            if x != '0' {
                format!("{}.{} thousand", num3.chars().nth(0).unwrap(), x)
            } else {
                format!("{} thousand", num3.chars().nth(0).unwrap())
            }
        }
        5 => {
            let x = num3.chars().nth(2).unwrap();
            if x != '0' {
                format!("{}.{} thousand", num3.get(0..2).unwrap(), x)
            } else {
                format!("{} thousand", num3.get(0..2).unwrap())
            }
        }
        6 => {
            let x = num3.chars().nth(3).unwrap();
            if x != '0' {
                format!("{}.{} thousand", num3.get(0..3).unwrap(), x)
            } else {
                format!("{} thousand", num3.get(0..3).unwrap())
            }
        }
        7 => {
            let x = num3.chars().nth(1).unwrap();
            if x != '0' {
                format!("{}.{} million", num3.chars().nth(0).unwrap(), x)
            } else {
                format!("{} million", num3.chars().nth(0).unwrap())
            }
        }
        8 => {
            let x = num3.chars().nth(2).unwrap();
            if x != '0' {
                format!("{}.{} million", num3.get(0..2).unwrap(), x)
            } else {
                format!("{} million", num3.get(0..2).unwrap())
            }
        }
        9 => {
            let x = num3.chars().nth(3).unwrap();
            if x != '0' {
                format!("{}.{} million", num3.get(0..3).unwrap(), x)
            } else {
                format!("{} million", num3.get(0..3).unwrap())
            }
        }
        10 => {
            let x = num3.chars().nth(1).unwrap();
            if x != '0' {
                format!("{}.{} billion", num3.chars().nth(0).unwrap(), x)
            } else {
                format!("{} billion", num3.chars().nth(0).unwrap())
            }
        }
        11 => {
            let x = num3.chars().nth(2).unwrap();
            if x != '0' {
                format!("{}.{} billion", num3.get(0..2).unwrap(), x)
            } else {
                format!("{} billion", num3.get(0..2).unwrap())
            }
        }
        12 => {
            let x = num3.chars().nth(3).unwrap();
            if x != '0' {
                format!("{}.{} billion", num3.get(0..3).unwrap(), x)
            } else {
                format!("{} billion", num3.get(0..3).unwrap())
            }
        }
        13 => {
            let x = num3.chars().nth(1).unwrap();
            if x != '0' {
                format!("{}.{} trillion", num3.chars().nth(0).unwrap(), x)
            } else {
                format!("{} trillion", num3.chars().nth(0).unwrap())
            }
        }
        14 => {
            let x = num3.chars().nth(2).unwrap();
            if x != '0' {
                format!("{}.{} trillion", num3.get(0..2).unwrap(), x)
            } else {
                format!("{} trillion", num3.get(0..2).unwrap())
            }
        }
        15 => {
            let x = num3.chars().nth(3).unwrap();
            if x != '0' {
                format!("{}.{} trillion", num3.get(0..3).unwrap(), x)
            } else {
                format!("{} trillion", num3.get(0..3).unwrap())
            }
        }
        16 => {
            let x = num3.chars().nth(1).unwrap();
            if x != '0' {
                format!("{}.{} quadrillion", num3.chars().nth(0).unwrap(), x)
            } else {
                format!("{} quadrillion", num3.chars().nth(0).unwrap())
            }
        }
        17 => {
            let x = num3.chars().nth(2).unwrap();
            if x != '0' {
                format!("{}.{} quadrillion", num3.get(0..2).unwrap(), x)
            } else {
                format!("{} quadrillion", num3.get(0..2).unwrap())
            }
        }
        18 => {
            let x = num3.chars().nth(3).unwrap();
            if x != '0' {
                format!("{}.{} quadrillion", num3.get(0..3).unwrap(), x)
            } else {
                format!("{} quadrillion", num3.get(0..3).unwrap())
            }
        }
        19 => {
            let x = num3.chars().nth(1).unwrap();
            if x != '0' {
                format!("{}.{} quintillion", num3.chars().nth(0).unwrap(), x)
            } else {
                format!("{} quintillion", num3.chars().nth(0).unwrap())
            }
        }
        20 => {
            let x = num3.chars().nth(2).unwrap();
            if x != '0' {
                format!("{}.{} quintillion", num3.get(0..2).unwrap(), x)
            } else {
                format!("{} quintillion", num3.get(0..2).unwrap())
            }
        }
        _ => num3,
    }
}

pub(crate) fn home(base: String, stats: Option<Stats>, days: u32) -> String {
    let data: PolyfillData =
        serde_json::from_str(include_str!("json/library-3.111.0.json")).unwrap();
    let mut aliases: IndexMap<String, Vec<String>> = data.polyfill_aliases.into_iter().collect();
    aliases.sort_keys();
    layout(html! {
        header {
            div class="container" {
                h1 {"Upgrade the web. Automatically.";}
                p {
                    "Delivers only the polyfills required by the user's web browser."
                }
            }
        }
        @if let Some(stats) = stats {
            article class="container" style="text-align: center;" {
                div class="grid" {
                    section {
                        span style="--font-size: 2rem;--typography-spacing-vertical: 3rem;--font-weight: 700;--color: var(--h1-color);color: var(--color);font-weight: var(--font-weight);font-size: var(--font-size);font-family: var(--font-family);" {
                            // (stats.requests.separate_with_commas())
                            (humanize(stats.requests))
                        }
                        br;
                        span {"requests served over last "(days)" days."}
                    }
                    section {
                        span style="--font-size: 2rem;--typography-spacing-vertical: 3rem;--font-weight: 700;--color: var(--h1-color);color: var(--color);font-weight: var(--font-weight);font-size: var(--font-size);font-family: var(--font-family);" {
                            (maud::display(format_size(stats.bandwidth, DECIMAL)))
                        }
                        br;
                        span {"worth of polyfills served over last "(days)" days."}
                    }
                }
                div {
                    p {
                        "Proudly sponsored by "
                    }
                    a href="https://www.fastly.com/products/edge-compute" {
                        svg height="60" viewBox="0 0 1555 602" width="155" xmlns="http://www.w3.org/2000/svg" {
                            g fill="#ff282d" {
                                path d="M1058.1 61.93v405.5h121.8v-61.96h-40.24V.17h-81.57zM.44 405.47h41.4V209.84H.44v-53.8l41.4-6.81V94.76C41.84 28.8 56.2.16 140.33.16c18.17 0 39.7 2.69 58.56 6.09l-11.17 66.31c-12.77-2.02-19.1-2.38-27.18-2.38-29.64 0-37.13 2.95-37.13 31.91v47.13h61.54v60.62H123.4v195.63h40.97v61.94l-163.94.02v-61.96zm1016.18-19.58c-12.76 2.7-23.92 2.37-32 2.57-33.55.82-30.65-10.2-30.65-41.85V209.84h63.87v-60.61h-63.87V.16h-81.56v363.36c0 71.34 17.6 103.9 94.34 103.9 18.17 0 43.14-4.67 62-8.71zm506.68 19.91a30.8 30.8 0 0 1 31.02 30.77 30.8 30.8 0 0 1-31.02 30.76 30.72 30.72 0 0 1-30.94-30.76 30.73 30.73 0 0 1 30.94-30.77m0 56.7a25.95 25.95 0 0 0 25.83-25.93c0-14.25-11.6-25.58-25.83-25.58a25.59 25.59 0 0 0-25.76 25.58c0 14.23 11.52 25.92 25.76 25.92m5.71-10.8-6.24-9.15h-4.3v9.14h-6.95v-30.24h12.65c7.47 0 12.13 3.78 12.13 10.47 0 4.91-2.46 8.26-6.32 9.4l7.56 10.37H1529zm-10.55-15.22h5.54c3.17 0 5.28-1.22 5.28-4.56 0-3.16-2.11-4.4-5.1-4.4h-5.72v8.96zM770.6 209.72v-10.85c-24.68-4.5-49.19-4.56-62.48-4.56-37.95 0-42.58 20.13-42.58 31.04 0 15.43 5.26 23.78 46.39 32.77 60.1 13.5 120.47 27.57 120.47 102.1 0 70.69-36.38 107.2-112.95 107.2-51.24 0-100.95-11-138.89-20.62v-60.91h61.77l-.04 10.82c26.58 5.13 54.45 4.62 69 4.62 40.49 0 47.04-21.77 47.04-33.34 0-16.06-11.62-23.77-49.57-31.47-71.5-12.22-128.24-36.63-128.24-109.26 0-68.74 45.98-95.71 122.55-95.71 51.87 0 91.32 8.04 129.27 17.67v60.5zm-375.27 53.54-6.2-6.2-31.53 27.46a15 15 0 0 0-5.07-.9c-8.5 0-15.39 7.1-15.39 15.83 0 8.75 6.89 15.83 15.4 15.83 8.5 0 15.4-7.08 15.4-15.83 0-1.66-.25-3.26-.72-4.76z" {};
                                path d="m520.26 385.89-.05-253.81h-81.56v23.8a166.73 166.73 0 0 0-55.46-20.98h.46v-28.16h9.95V86.02h-82.12v20.72h9.95v28.16h.56c-78 14.36-137.12 82.67-137.12 164.83 0 92.6 75.06 167.66 167.65 167.66 31.6 0 61.16-8.75 86.39-23.95l14.7 23.98h86.15V385.9h-19.5zm-162.85-.19v-9.58h-9.77v9.56a86.29 86.29 0 0 1-81.08-81.57h9.72v-9.77h-9.66a86.28 86.28 0 0 1 81.02-80.92v9.61h9.77v-9.64a86.28 86.28 0 0 1 81.26 78.51v2.8h-9.8v9.78h9.8v2.67a86.28 86.28 0 0 1-81.26 78.55zm1028.93-236.48h168.22v60.56h-40.2L1411.17 463.6c-29.57 71.3-78.13 138.4-152.1 138.4-18.18 0-42.4-2.01-59.18-6.04l7.37-74.03c10.78 2.01 24.89 3.34 32.3 3.34 34.3 0 72.98-21.25 85.09-58.26l-104.54-257.23h-40.21v-60.55h168.3v60.55H1308l59.23 145.7 59.22-145.7h-40.12v-60.56z" {};
                            }
                        }
                    }
                }
            }
        }
        main class="container" {
            form {
                label for="bundle" {
                    "Your polyfill bundle";
                    output {
                        pre {
                            code id="polyfill-bundle-url" {
                                (base) "/v3/polyfill.min.js"
                            }
                        }
                    }
                }
                div class="grid" {

                    label for="library-version" {
                        "Polyfill Library Version";
                        select id="library-version" {
                            option value="3.111.0" selected="" {"3.111.0";}
                            option value="3.110.1" {"3.110.1";}
                            option value="3.109.0" {"3.109.0";}
                            option value="3.108.0" {"3.108.0";}
                            option value="3.104.0" {"3.104.0";}
                            option value="3.103.0" {"3.103.0";}
                            option value="3.101.0" {"3.101.0";}
                            option value="3.98.0" {"3.98.0";}
                            option value="3.96.0" {"3.96.0";}
                            option value="3.89.4" {"3.89.4";}
                            option value="3.53.1" {"3.53.1";}
                            option value="3.52.3" {"3.52.3";}
                            option value="3.52.2" {"3.52.2";}
                            option value="3.52.1" {"3.52.1";}
                            option value="3.52.0" {"3.52.0";}
                            option value="3.51.0" {"3.51.0";}
                            option value="3.50.2" {"3.50.2";}
                            option value="3.48.0" {"3.48.0";}
                            option value="3.46.0" {"3.46.0";}
                            option value="3.42.0" {"3.42.0";}
                            option value="3.41.0" {"3.41.0";}
                            option value="3.40.0" {"3.40.0";}
                            option value="3.39.0" {"3.39.0";}
                            option value="3.34.0" {"3.34.0";}
                            option value="3.27.4" {"3.27.4";}
                        }
                    }

                    label for="callback" {
                        "Callback";
                        input type="text" id="callback" name="callback";
                    }

                }

                label for="filter-polyfills" {"Filter Polyfills";}
                input type="text" id="filter-polyfills" name="filter-polyfills";
                small{"Filter the polyfills in the \"Available Polyfills\" list."}

                fieldset {
                    legend {"Available Polyfills";}
					small{"Check the boxes of the polyfills or polyfill-sets you want to have in your bundle.";}

                    div id="features-list" {
                        @for alias in &aliases {
                            div class="feature" {
                                label for=(alias.0) {
                                    input type="checkbox" id=(alias.0) name=(alias.0);
                                    (alias.0);
                                }
                                details class="alias" {
                                    summary{"Included Polyfills";}
                                    ul{
                                        @for polyfill in alias.1 {
                                            li{(polyfill);}
                                        }
                                    }
                                }
                            }
                        }
                        @for polyfill in &data.polyfills {
                            div class="feature" {
                                label for=(polyfill) {
                                    input type="checkbox" id=(polyfill) name=(polyfill);
                                    (polyfill);
                                }
                            }
                        }

                    }
                }
            }
            button class="contrast switcher theme-switcher" aria-label="Turn off dark mode" {
                i{"Turn off dark mode"}
            }
        }
        script {
            (Raw(include_str!("builder.js").to_string()));
        }
    }).into_string()
}

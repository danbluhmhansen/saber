use std::{error::Error, fs};

use derive_builder::Builder;
use maud::{html, Markup, Render, DOCTYPE};

fn index() -> Result<Markup, Box<dyn Error>> {
    Ok(html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width initial-scale=1";
                link rel="icon" type="image/svg+xml" href="data:image/svg+xml;base64,PHN2ZyB4bWxucz0naHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmcnIHdpZHRoPScxZW0nIGhlaWdodD0nMWVtJyB2aWV3Qm94PScwIDAgNTEyIDUxMic+PHBhdGggZmlsbD0nY3VycmVudENvbG9yJyBkPSdNMTA3LjkzOCAxNy4zMTNjLTY0LjA1LS41OTQtMTM5LjgyIDE3OC4zMDUtNDAuMTU3IDMzNy43OEM0Ni40MiAxOTguMjA4IDg5LjMyIDEzNS4yOTIgMTIwLjIyIDk4Ljc1Yy01LjIzNyA0NC4xNzQgMy45NjYgMTA0LjY0MiAzNS4xODYgMTcxLjMxM2MtMy44ODMtOTkuNDE3IDIwLjkzLTE1NS4yMDUgMzcuNjg4LTE4OC44MTNjLTIuNTMyIDQwLjIzNSA5LjA5NiA5OC42MzUgMzIuMjggMTU4LjY4OGM2LjM3My03NS4zMDggMjMuMjg3LTEyNi42OSAzMC41NjQtMTYwLjA5NGM3LjI3NiAzMy40MDMgMjQuMTkgODQuNzgyIDMwLjU2MiAxNjAuMDk0YzIzLjIwMy02MC4xIDM0LjgwNi0xMTguNTQ0IDMyLjI1LTE1OC43ODJjMTYuNzU3IDMzLjU5OCA0MS42MzcgODkuMzk0IDM3Ljc1IDE4OC45MDZjMzEuMjItNjYuNjcgNDAuNDI0LTEyNy4xMzggMzUuMTg4LTE3MS4zMTJjMzAuOSAzNi41NCA3My43NjUgOTkuNDYyIDUyLjQwNiAyNTYuMzQ0YzExOC4xMi0xODkuMDA4LTEwLjE1LTQwNS4zLTczLjMxMy0zMTcuNzJhNTUuNDM5IDU1LjQzOSAwIDAgMC0xLjcxNyAyLjVjLTEyLjc5OC0xNC4wNzMtMjkuODc3LTE4LjcwNy00OS4xMjUtMTEuMDNjLTcuMzk3IDIuOTUtMTEuOCA2LjExNi0xMy45NyA5LjkwNmMtNy44MDYtOS42NS0xOS4xOTUtMTMuMjUtMzQuNDY4LTguOTdjLTcuNjcgMi4xNS0xMi42MzggNS41NzMtMTUuNTYzIDEwLjQ3Yy0yLjkyNi00Ljg5LTcuOS04LjMyLTE1LjU2Mi0xMC40N2MtMTUuMjU0LTQuMjc2LTI2LjYzLS42ODQtMzQuNDM4IDguOTRjLTIuMTc1LTMuNzgtNi42Mi02LjkzMy0xNC05Ljg3NmMtMTkuMjQ4LTcuNjc3LTM2LjI5Ni0zLjA0My00OS4wOTMgMTEuMDNhNTQuNzQ0IDU0Ljc0NCAwIDAgMC0xLjcyLTIuNWMtOS44NjgtMTMuNjg0LTIxLjMyNS0xOS45NTItMzMuMTg2LTIwLjA2MnptMTIuMzEyIDI1NC4wM2MtMzguMjggMTQ4Ljk3IDE4LjQwNCAyNTEuNTg1IDczLjkzOCAyMTYuNjg4YzcuNTYtNC43NSAxMS4wNzMtOS4wOSAxMS41OTMtMTMuOTA1YzguMjQzIDExLjE5NCAyMC4zMyAxNS4xNDYgMzYuNDcgOS45MzhjNy4wMjYtMi4yNjggMTEuMzkzLTUuNjY2IDEzLjY4OC0xMC4zNzVjMi4yOTQgNC43MSA2LjY2IDguMTA3IDEzLjY4NyAxMC4zNzVjMTYuMTQgNS4yMDggMjguMjI3IDEuMjU2IDM2LjQ3LTkuOTM4Yy41MiA0LjgxNiA0LjAzMiA5LjE1NSAxMS41OTIgMTMuOTA2YzU1LjUzNCAzNC44OTggMTEyLjIxOC02Ny43MTYgNzMuOTM4LTIxNi42ODZjLTE1LjcgMTA5LjI4Ny01Ny41ODUgMTU1Ljc2Ni03Ni4zMTMgMTgxLjk3YzkuMTQ2LTM0Ljg3NSAzLjc4LTk0LjMxNi0xNC4yNS0xNjMuNTY0Yy0xMy45MzMgNzkuNTYtMzguMDY2IDEyOS42NDYtNDUuMTI1IDE1OS41NjNjLTcuMDYtMjkuOTE4LTMxLjE5Mi04MC4wMS00NS4xMjUtMTU5LjU2M2MtMTguMDMgNjkuMjQ4LTIzLjM2NCAxMjguNjktMTQuMjE4IDE2My41NjNjLTE4LjcyNy0yNi4yMDMtNjAuNjQ1LTcyLjY4Mi03Ni4zNDQtMTgxLjk3eicvPjwvc3ZnPg==";
                link rel="stylesheet" type="text/css" href="site.css";
            }

            body class="text-white bg-slate-900" {
                (CardBuilder::default().id("foo").title("Title").tag("Tag").render()?)
            }
        }
    })
}

#[derive(Builder)]
struct Card<'a> {
    #[builder(setter(into))]
    id: &'a str,
    #[builder(setter(into))]
    title: &'a str,
    #[builder(setter(into, strip_option), default)]
    tag: Option<&'a str>,
}

impl CardBuilder<'_> {
    fn render(&self) -> Result<Markup, Box<dyn Error>> {
        let Card { id, title, tag } = self.build()?;
        Ok(html! {
            #(id) class="grid grid-rows-2 bg-gradient-to-br from-red-700 to-yellow-700 rounded-lg divide-y-4 divide-amber-300 aspect-[8/11] w-[318px]" {
                div class="flex flex-col justify-between p-2" {
                    div class="flex justify-between" {
                        div class="flex flex-col" {
                            span class="text-2xl text-center" { "1" }
                            (icon_axe_sword(2))
                        }
                        div {
                            div class="flex flex-row-reverse gap-1 items-center" {
                                span class="text-xl align-middle" { "0" }
                                (icon_lightning(1.25))
                            }
                        }
                    }
                    div class="grid grid-flow-col justify-stretch" {
                        div {}
                        div class="text-lg text-center" {
                            @if let Some(tag) = tag { (tag) }
                        }
                        div class="text-right" {}
                    }
                }
                div class="p-2 text-center text-black bg-clip-content bg-amber-100" {
                    h1 class="text-xl font-bold" { (title) }
                    "Assumenda in sit sit. Consequatur reiciendis architecto tempora autem vitae consectetur et. Aut voluptatem quia assumenda aut et omnis iusto."
                }
            }
        })
    }
}

fn icon_axe_sword(size: impl Render) -> Markup {
    html! {
        svg xmlns="http://www.w3.org/2000/svg" width={(size) "em"} height={(size) "em"} viewBox="0 0 512 512" {
            path fill="currentColor" d="m329.5 29.12l-8.1 11.4L359 67.16l8.1-11.44zm-88 5.04l24.2 45.36l1.8 1.29l14.8-40.36zm57.6 12.63l-16.4 44.8l40.7 28.81l35.3-31.54c-.9-.58-1.9-1.19-2.8-1.84zM59.83 48.56l10.84 45.83l29.63 2.6l2.7-29.63zM470.9 75.41c-5.6 4.71-12.2 8.59-19.5 11.74c5 46.45-14.7 83.45-45.2 109.75c-26.5 22.9-60.9 38.4-95 47.9c-2.5 4.8-5 9.2-7.4 13.1c41.5 5.4 93.2-21.2 129.2-60c19.8-21.3 34.8-45.9 41.1-69.2c5.2-19.4 4.7-37.42-3.2-53.29m-351.3 8.71l-3 32.48l-32.35-2.9l226.55 271l20-16.7l15.3-12.8zM434 93.09c-4.2 1-8.5 2-12.8 2.7c-14.9 2.5-30.1 3.1-43.5.3l-41 36.61c4 7 5 15.7 4.5 24.5c-.6 12.6-4.3 26.7-9.3 40.9c-3 8.3-6.3 16.6-9.9 24.6c26.9-9.2 52.6-22.3 72.5-39.4c26.2-22.8 42.5-51.6 39.5-90.21M274 107.4l-51.2 72.2l30.6 36.5l58.2-82.1zM173.8 248.8L34.53 445.2l37.53 26.6L204.3 285.3zm233 79.2L273.3 439.5l19.2 23.1L426 351zm-18.3 77.9l-35.3 29.4l39.7 47.6l35.3-29.4z";
        }
    }
}

fn icon_lightning(size: impl Render) -> Markup {
    html! {
        svg xmlns="http://www.w3.org/2000/svg" width={(size) "em"} height={(size) "em"} viewBox="0 0 512 512" {
            path fill="currentColor" d="M29.805 29.777L242.14 209.55H118.712l112.54 86.784H95.995l225.656 174.012l-81.537-116.05l66.487.143l179.185 138.175l-171.96-244.746h84.568L248.082 29.776z";
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    fs::write("dist/index.html", index()?.into_string())?;

    Ok(())
}

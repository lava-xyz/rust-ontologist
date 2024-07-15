use anyhow::anyhow;
use serde::Serialize;

use crate::output::cytoscape::Repr;

const INDEX_TEMPLATE: &str = r##"
    <!DOCTYPE html>
    <html lang="en">

    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>{{title}}</title>
        <!--
        Graph theory (network) library for visualisation and analysis:
        https://js.cytoscape.org/
        -->
        <script src="https://cdnjs.cloudflare.com/ajax/libs/cytoscape/3.23.0/cytoscape.min.js"></script>

        <!--
        The Klay layout algorithm for Cytoscape.js:
        https://github.com/cytoscape/cytoscape.js-klay
        -->
        <script src="https://unpkg.com/klayjs@0.4.1/klay.js"></script>
        <script
            src="https://cdn.jsdelivr.net/gh/cytoscape/cytoscape.js-klay@9fab410e4823268b5fcf877b1f5af2798dd98371/cytoscape-klay.js"></script>

        <!--
        Bird's eye view pan and zoom control for Cytoscape.js:
        https://github.com/cytoscape/cytoscape.js-navigator
        -->
        <!-- <link
            href="https://cdn.jsdelivr.net/gh/cytoscape/cytoscape.js-navigator@a7507f067eb4c7f3c11ec298992ec0c578b8c175/cytoscape.js-navigator.css"
            rel="stylesheet" type="text/css" />
        <script
            src="https://cdn.jsdelivr.net/gh/cytoscape/cytoscape.js-navigator@a7507f067eb4c7f3c11ec298992ec0c578b8c175/cytoscape-navigator.js"></script> -->
    </head>
    <style>
        #cy {
            width: 100%;
            height: 100%;
            position: absolute;
            top: 0px;
            left: 0px;
            overflow: hidden;
        }
    </style>

    <body>
        <div id="cy"></div>
        <script type="module">
            const graph = {{ json_to_str graph }};
                    var cy = cytoscape({
                        container: document.getElementById("cy"),
                        elements: graph,
                        wheelSensitivity: 0.1,
                        layout: {
                            name: "klay",
                            avoidOverlap: true,
                            nodeDimensionsIncludeLabels: true
                        },
                        style: [
                            {
                                selector: "node",
                                style: {
                                    "label": "data(name)",
                                    "font-family": "monospace"
                                }
                            },
                            {
                                selector: ".vertex-package",
                                style: {
                                    "font-size": "24px"
                                }
                            },
                            {
                                selector: ".vertex-non-package",
                                style: {
                                    "min-zoomed-font-size": "12px"
                                }
                            },
                            {
                                selector: "edge",
                                style: {
                                    "curve-style": "unbundled-bezier",
                                    "line-cap": "round",
                                    "target-arrow-shape": "triangle",
                                }
                            },
                            {
                                selector: ".edge-red",
                                style: {
                                    "line-color": "#9B2335",
                                    "target-arrow-color": "#9B2335"
                                }
                            },
                            {
                                selector: ".edge-blue",
                                style: {
                                    "line-color": "#34568B",
                                    "target-arrow-color": "#34568B"
                                }
                            },
                            {
                                selector: ".edge-green",
                                style: {
                                    "line-color": "#2E8B57",
                                    "target-arrow-color": "#2E8B57"
                                }
                            },
                            {
                                selector: ".edge-violet",
                                style: {
                                    "line-color": "#6B5B95",
                                    "target-arrow-color": "#6B5B95"
                                }
                            },
                            {
                                selector: ".edge-orange",
                                style: {
                                    "line-color": "#DD6E0F",
                                    "target-arrow-color": "#DD6E0F"
                                }
                            },
                            {
                                selector: ".edge-purple",
                                style: {
                                    "line-color": "#663399",
                                    "target-arrow-color": "#663399"
                                }
                            },
                            {
                                selector: ".edge-plum",
                                style: {
                                    "line-color": "#DDA0DD",
                                    "target-arrow-color": "#DDA0DD"
                                }
                            }
                        ]
                    });
        </script>
    </body>

    </html>
"##;

pub fn render_index(title: impl Into<String>, graph: &Repr) -> anyhow::Result<String> {
    #[derive(Serialize)]
    struct Payload<'graph> {
        title: String,
        graph: &'graph Repr,
    }

    let payload: Payload = Payload { title: title.into(), graph };

    let mut hb = handlebars_misc_helpers::new_hbs();
    handlebars_misc_helpers::setup_handlebars(&mut hb);
    hb.render_template(INDEX_TEMPLATE, &payload)
        .map_err(|e| anyhow!("Failed to render template ({})", e.reason()))
}

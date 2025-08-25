#!/bin/bash
mdbook build
mdslides --template ./template.html --output-dir ./slides --mdbook-path . --index-template ./index-template.html 

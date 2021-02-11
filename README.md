![Build](https://github.com/ylst/rusty-rest/workflows/Build/badge.svg)
# RustyRest

RustyRest is a simple REST server written in Rust using the Rocket web framework. I use this application mainly for testing purposes.

## Endpoints

The REST server returns a response on the following endpoints:

### localhost/ (Index)

Returns the line "RustyRest is running!".

### localhost/makestring/<input>

Return the line "You entered: '{}'" formatted with the text given as input in the URL.

### localhost/image

Returns a very rusty JPEG image.

### localhost/video

Returns a video clip.

## Attribution

[Image](https://www.pexels.com/photo/aged-brown-chain-close-114108/) | 
[Video](https://www.pexels.com/video/aerial-view-of-a-town-in-tennessee-2282013/)

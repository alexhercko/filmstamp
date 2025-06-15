# filmstamp

CLI tool to add film-like timestamps to images based on the image's EXIF data.

## Features
- Extracts timestamp from image EXIF data (mainly from the `DateTimeOriginal` field)
- Writes the timestamp to the image using default settings with a default font

## CLI reference

| Argument   | Description         |
|------------|---------------------|
| \<input\>  | Path to input image |

| Option       | Description          |
|--------------|----------------------|
| -o, --output | Path to output image |

## Credits

Author [@alexhercko](https://github.com/alexhercko)
Default font [Digital-7 Sizenko Alexander (Style-7)](http://www.styleseven.com)

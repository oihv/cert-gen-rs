# Cert Gen
A certificate generator utility program.

## How To Use (WIP)
Add a placeholder for the text that wants to be automated.

Include a data source for the placeholder to find the value for each individual. The column must match to the name of the placeholder.

## TODO
- [x] Fixed image to be able to refresh when detecting new version
- [x] Create template creation, with preview of the text (now called placeholder)
- [x] Add multiple placeholder, edit, increase font size, and delete them
- [x] File picker to pick the certificate template
- [x] Add new external font
- [x] Change the font of the placeholder (~needs file picker~ it's called a combo box in egui), 
- [ ] Load different type of files for the information template (txt,csv)
- [ ] File name template
- [ ] Find how to find the position value of the text relative to the image
- [ ] Find how to get imageproc to also interpret the installed font. (We must load it twice. egui and ab_glyph)
- [ ] Get the first batch of certificate made by this project, hurray.
- [ ] Preview mode (check the first value in the data source and display it in a popup window)
- [ ] Add automatic values too (e.g. date, but this can also be automated inside excel anyway)
- [ ] Make control panel more neat by placing them in a grid instead
- [x] Set alignment of the placeholder (vertically centered / horizontally centered)
- [x] wrap mode (justify, align left/right)
- [ ] Load fonts installed in system -> I read that there's no crate that can help with that, yet. (https://github.com/emilk/egui/issues/749)
- [ ] Other styling functions (bold, italic) -> For now can be deferred to the user to select the style based on the font variation they install. -> But this function is really needed, think of a certificate where you need a whole paragraph to be typed (e.g. Mr. Tang as a **Coordinator of Media Division**), we would want this capability in the future.
- [ ] Caching (installed fonts? project file to store the template image and the placeholders?)
- [ ] Web version release
- [ ] Since egui uses their own font for it's installation, let's just load a new default font for both egui and for placeholder (install a nerd font too so emojis are supported)

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
- [x] Load different type of files for the information template (txt,csv) (any text file comma separated is acceptable)
- [ ] File name template
- [x] Find how to find the position value of the text relative to the image (apparently this is pretty weird, since I have to first set egui to display the image to its original size, before being able to mirror the drawing of the text on top of the image with imageproc. Previous attempt with finding ratio fails a bit with the width of the text by imageproc, I think it is because the floating point precision can't get the exact ratio)
- [x] Find how to get imageproc to also interpret the installed font. (We must load it twice. egui and ab_glyph) (built a hashmap in the app data, font_vec_handles, that uses the display format of the font_family, and it points to the fontvec that got processed at the same time when local fonts are also installed)
- [x] Get the first batch of certificate made by this project, hurray.
- [ ] Preview mode (display the values of the source in the image)
- [ ] Add automatic values too (e.g. date, but this can also be automated inside excel anyway)
- [x] Make control panel more neat by placing them in a grid instead
- [x] Set alignment of the placeholder (vertically centered / horizontally centered)
- [x] wrap mode (justify, align left/right)
- [x] Color picker
- [ ] Load fonts installed in system -> I read that there's no crate that can help with that, yet. (https://github.com/emilk/egui/issues/749)
- [ ] Other styling functions (bold, italic) -> For now can be deferred to the user to select the style based on the font variation they install. -> But this function is really needed, think of a certificate where you need a whole paragraph to be typed (e.g. Mr. Tang as a **Coordinator of Media Division**), we would want this capability in the future.
- [ ] Caching (installed fonts? project file to store the template image and the placeholders? Prompt if user exits without saving the project?)
- [ ] Web version release
- [x] Since egui uses their own font for it's installation, let's just load a new default font for both egui and for placeholder (install a nerd font too so emojis are supported)
- [ ] Warning when there's column in source data that doesn't have the corresponding placeholder, prevent generation when there's still issue
- [ ] think about how to propagate the error, handle it, instead of just panicking.
- [x] Handle alignment when drawing
- [ ] Add a notification popup that shows that some action is completed (loading a file, loading source, generating)
- [x] Shows loading bar when generating instead of just freezing
- [ ] CLI version??? (what for? ofc for fast automation, pass in the template file, comma separated value, and boom)

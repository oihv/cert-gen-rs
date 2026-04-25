# Cert Gen
A certificate generator utility program.

## How To Use
1. Image
Choose an image as a template.

2. Placeholder
Add a placeholder for the text that wants to be automated.

Variable are specified with curly braces ({}). For example "Date: {date}", means "Date: " is a string constant/literal, and `date` is a variable that will be searched from the source data.

3. Data Source
Choose a data source for the placeholder to find the value for each individual. The variable should match a column in the source, or else generation can't be done.

4. Generate!
Press the generate button in the Control Panel. And your certificates is ready to be distributed.

## Features
- Additional Fonts
With `Install Local Fonts` button, you can add `.otf` `.ttf` fonts into the application.

After that, choose the newly added font for which placeholder you want.

- Workspace saving (WIP)
Save your workspace (including placeholders, data source, template image), for future usage. Workspace is saved in a toml format.

- CLI mode (WIP)
With the workspace file, you can generate images quickly without accessing the GUI.

## TODO
- [x] Fixed image to be able to refresh when detecting new version
- [x] Create template creation, with preview of the text (now called placeholder)
- [x] Add multiple placeholder, edit, increase font size, and delete them
- [x] File picker to pick the certificate template
- [x] Add new external font
- [x] Change the font of the placeholder (~needs file picker~ it's called a combo box in egui), 
- [x] Load different type of files for the information template (txt,csv) (any text file comma separated is acceptable)
- [x] File name template
- [x] Output directory picker
- [x] Find how to find the position value of the text relative to the image (apparently this is pretty weird, since I have to first set egui to display the image to its original size, before being able to mirror the drawing of the text on top of the image with imageproc. Previous attempt with finding ratio fails a bit with the width of the text by imageproc, I think it is because the floating point precision can't get the exact ratio)
- [x] Find how to get imageproc to also interpret the installed font. (We must load it twice. egui and ab_glyph) (built a hashmap in the app data, font_vec_handles, that uses the display format of the font_family, and it points to the fontvec that got processed at the same time when local fonts are also installed)
- [x] Get the first batch of certificate made by this project, hurray.
- [x] Make control panel more neat by placing them in a grid instead
- [x] Set alignment of the placeholder (vertically centered / horizontally centered)
- [x] wrap mode (justify, align left/right)
- [x] Color picker
- [x] Since egui uses their own font for it's installation, let's just load a new default font for both egui and for placeholder (install a nerd font too so emojis are supported)
- [x] Handle alignment when drawing
- [x] Add a table that shows the values in the loaded source
- [x] Change placeholder to force the use of {} to specify syntax instead.
- [ ] think about how to propagate the error, handle it, instead of just panicking. -> Custom error types, handle it in the main buffer by showing it in the main UI, return early if there's an error, inspired by Green Tea Coding video!)
- [ ] Warning when there's column in source data that doesn't have the corresponding placeholder, prevent generation when there's still issue (Can add it to the table)
- [ ] Add a notification popup that shows that some action is completed (loading a file, loading source, generating)
- [x] Shows loading bar when generating instead of just freezing
- [ ] CLI version??? (what for? ofc for fast automation, pass in the template file, comma separated value, and boom)
- [ ] Preview mode (display the values of the source in the image)
- [ ] Add automatic values too (e.g. date, but this can also be automated inside excel anyway)
- [ ] Other styling functions (bold, italic) -> For now can be deferred to the user to select the style based on the font variation they install. -> But this function is really needed, think of a certificate where you need a whole paragraph to be typed (e.g. Mr. Tang as a **Coordinator of Media Division**), we would want this capability in the future.
- [ ] Caching (installed fonts? project file to store the template image and the placeholders? Prompt if user exits without saving the project?)
- [ ] Web version release
- [ ] Load fonts installed in system -> I read that there's no crate that can help with that, yet. (https://github.com/emilk/egui/issues/749)
- [ ] String escaping, what if the user wants to add curly braces as a literal? (But who will do that anyway for a certificate)
- [ ] When changing text align, position changes unexpectedly

# Zenless Disk OCR

该项目目标是利用Tesseract OCR技术，实现对《绝区零》驱动盘词缀的自动识别、保存和（可能有）评分。

该项目处于非常早期的开发阶段，开发主要针对**简体中文**语言，**全局细体**字体，**键鼠**或**控制器**布局，屏幕分辨率为**16:9**。其他语言、字体、操作布局和屏幕比例的支持，待软件开发基本完成，功能稳定后进行。

该项目的发布版预计将提供一个压缩包，内含一个可执行文件，以及Tesseract数据。运行可执行文件后，软件将要求用户框定一个范围，并监听快捷键，用户按下快捷键后对区域进行截屏和OCR，根据OCR结果生成驱动盘数据，保存到一个Sqlite数据库中。可通过启动参数将该数据库导出为其他表格格式。

开发进度：

- ✘ OCR范围选择和快捷键监听
- ✘ OCR与预处理
- ✘ 解析驱动盘数据
- ✔ 保存数据
- ✔ 导出数据
- ✘ 测试
- ✘ 驱动盘评分

---

The project aims at automatically scanning, saving and (possibly) rating affixes for Drive Disks in the game *Zenless Zone Zero*, via OCR techniques.

This project is at a very early stage in development. Now we are currently working with **Simplified Chinese** language, **Global Light** font scheme, **Mouse And Keyboard** or **Controller** layout, **16:9** resolutions. Support for other languages, fonts, control layouts or screen ratio won't start until the development is near complete and features are stable.

A release of the project is estimated to provide an archive containing an executable and Tessearct data. On running the executable the user will be asked to draw a rectangle, which will be snapshot and OCR'd when the user presses a hotkey. The data will be generated based on the result of OCR and saved to a Sqlite database. The database can be exported as other sheet formats through arguments.

Progress:

- ✘ OCR Rectangle Selection
- ✘ OCR And Preprocessing
- ✘ Parsing Disk Data
- ✔ Saving Data
- ✔ Exporting Data
- ✘ Testing
- ✘ Rating Disks
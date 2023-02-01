# Ezio Sweet's 自用QQ Bot

基于[rust_proc_qq](https://github.com/niuhuan/rust_proc_qq)

## 目前指令

+ 随机涩图
  + e.g. ``随机涩图``
  + 返回随机一张涩图
+ 搜涩图
  + e.g. ``搜涩图 102106175``，后为P站图片pid
  + 返回搜索的图片

## 使用方式

直接从release下载相应的文件打开即可

PS：倘若在服务器上使用，请先在客户端生成``device.json``和``session.token``，复制到服务端二进制文件同目录

原因是QQ不能远距离扫码登录，服务器和手机位置不相同则只能通过token登录。
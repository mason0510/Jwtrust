## http的验证
1.session
2.token认证

##流程
用户名和密码请求服务器
服务器验证用户名和密码
如果验证成功，服务器返回一个token，用于后续的验证
如果验证失败，服务器返回一个错误信息
客户端每次请求时，都需要携带token，服务器验证token是否有效
服务端验证token是否有效，如果有效，则继续请求，如果无效，则返回错误信息 跳到第一步


##问题
token必须放在auth里不能放在header里赋值

##rust相关
见代码

##测试用例
test.rs

##测试接口
http://127.0.0.1:8888/jwt/get
http://127.0.0.1:8888/jwt/val


##相关
https://blog.csdn.net/qq_42970717/article/details/122961694
https://icode.best/i/99011745485102
https://users.rust-lang.org/t/how-to-fix-doesnt-implement-std-debug/70834/4

##y运行
cargo run 
rm cargo.lock

##crate原文地址
https://learnku.com/articles/31161

# 编码规范
1. 不使用 unwrap, 针对result使用? 与 AppError 传递错误并处理, 对Option 使用ok_or(NoneError)? 与 AppError 传递错误,  只针对fatal 错误使用expect
2. test 可以用unwrap
3. 使用rustfmt 格式化后提交
4. 使用clippy检查并且不要有warning , 如果有,留下解释
5. unsafe 必须有 解释
6. 不添加 macro_rules 宏, 尽量使用derive宏
7. 尽量使用result,而不是panic, test 
8. 小心使用let _ = , 这会立即drop , let _x = 会正常drop
9. 减少所有if 与 for 嵌套
10. 尽量不增加feature
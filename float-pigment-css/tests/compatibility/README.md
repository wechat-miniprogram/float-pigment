# css serde 兼容性测试

> 用于测试 css bincode 是否具有前/后兼容性

## 目录说明

## 向前兼容性

检验新的 deserializer 是否能正确解开历史版本的的 bincode

* 用当前版本的 deserializer 去解 bincode_cache 中的所有 bincode，检查结果是否符合预期

* 向 bincode_cache 写入当前 serialize 的 bincode，供下一个版本测试

## 向后兼容性

检验历史版本的 deserializer 是否能兼容更新编出来的 bincode

* 用当前版本编出来的 bincode 给所有的历史 deserializer 去解，检查结果是否符合预期


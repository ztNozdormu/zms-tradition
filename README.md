# zms-tradition
Legends of mountains and seas ；crypto trading platform

## work plan

### 2024-11-18 
 1. 业务初步设计,创建项目设计文档
 2. 下一步设计选币机器人服务详细 
### 2024-11-20
1. 初始化 cargo new --lib zms-tradition-proto-grpc-types 模块
2. 初始化 cargo new --lib zms-tradition-rest-types 模块
### 2024-11-21
1. 基础数据定义转换函数实现
2. 初始化 zms-tradition-rest-gateway-server 网关服务
3. 初始化 zms-tradition-gw3data-server 业务数据服务
4. 初始化 zms-tradition-picker-gbot  选币机器人服务 gbot：提供grpc服务 rbot:提供rest接口服务
### 2024-1-22
1. 网关服务增加启动逻辑，增加全局配置config.toml
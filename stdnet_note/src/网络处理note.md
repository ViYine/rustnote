# 网络处理的方式

bind(addr) -> listener(accept)-> stream(read,write),frame(在连接上做数据包的封装)
connect(addr) -> stream(read,write)

对于拿到的一个connect 如何在上面做封包，解包的操作？ -> Frame 相关的操作和接口。

协议的Frame 的形式：

1. 分隔符+消息数据: tokio-util:  LinesDelimited（处理 \r\n 分隔符
2. 长度+消息数据: tokio-util: LengthDelimited（处理长度分隔符）
3. 混用两者。

## loop+spawn



## loop+thread pool

## loop+thread pool + async

## loop+thread pool + async + queue

- loop + spawn 是最简单的网络处理方式；
    1. 需要考虑的问题：大量的连接会导致大量的线程；线程间数据的共享。

- loop + thread pool 是一个比较好的网络处理方式；
    1. 可能问题：线程池中的线程都在处理连接的时候，新的连接就会被阻塞；

- loop + thread pool + queue 是一个更好一点的网络处理方式；
    1. 可能问题：队列中的连接过多的时候，会导致内存的浪费。

处理网络连接时，我们需要考虑的问题：

1. 大量的连接会导致大量的线程-> 使用异步网络运行时：tokio；
2. 线程间数据的共享-> channel, `Arc<T>`, `Arc<RwLock<T>>`等, 用锁的时候需要考虑降低锁的粒度；
3. 线程池中的线程都在处理连接的时候，新的连接就会被阻塞；
4. 队列中的连接过多的时候，会导致内存的浪费。

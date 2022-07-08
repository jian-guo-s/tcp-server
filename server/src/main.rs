use std::net::{TcpListener, TcpStream};

//std::thread库的引入，对输入的每一个流创建一个线程
use std::thread;
use std::time;

//引入io库，为了处理错误
use std::io::{self, Read, Write};

//该函数用来处理client，流的格式或者说他的类型就是TcpStream
fn handle_client(mut stream: TcpStream) -> io::Result<()> {

    //创建一个叫buf的数组，内容为0，长度为512
    let mut buf = [0; 512];

    //该循环表示server端永久提供服务，因为默认服务器为永不关闭的
    for _ in 0..1000 {
        //从流里面读内容，读到buf中
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            //如果读到的为空（即0），则说明已经结束了
            return Ok(());
        }

        //否则把它写回去
        stream.write(&buf[..bytes_read])?;

        //调用sleep函数实现服务的间隔，间隔1s
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}

fn main() -> io::Result<()> {

    //定义一个listener，bind函数里面填写的是监听的的ip与端口号,?是一种简写，等价于except,unwrap
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    //创建一个容器，用来放线程的句柄
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    
    for stream in listener.incoming() {

        //转换一下stream流，出现问题，提示“失败”，没有问题，继续下面的操作
        let stream = stream.expect("failed");
        let handle = thread::spawn(move || {
            //对输入的每一个流来创建一个线程，利用必包进行一个处理
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        //把handle加到容器里面
        thread_vec.push(handle);
    }

    //此循环为了等待线程的结束
    for handle in thread_vec {

        //等待结束的具体实现
        handle.join().unwrap();
    }
    Ok(())
}

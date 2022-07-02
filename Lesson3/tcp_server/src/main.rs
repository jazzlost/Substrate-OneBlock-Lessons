
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

//Server端业务接口
fn handle_client(mut stream: TcpStream)
{
    //分配一个128位buff
    let mut data = [0 as u8; 128];

    //从TcpStream中读取Client端Request
    while match stream.read(&mut data)
    {   
        //读取结果模式匹配，read无异常
        Ok(size) => 
        {
            //将buff信息原封不动写回Response
            stream.write(&data[0..size]).unwrap();
            //继续监听Request
            true
        },
        //读取结果模式匹配，read异常
        Err(_) =>
        {
            //打印错误信息
            println!("Error Occured, Shut Down Connection With {}", stream.peer_addr().unwrap());
            //结束监听
            false
        }
    } {}
}

fn main() 
{
    //监听端口
    let listen_addr = "localhost:3333".to_string();
    //创建Server实例，绑定监听地址
    let listener = TcpListener::bind(listen_addr).unwrap();
    
    println!("Server Listening Port: 3333");

    //遍历所有此端口Connection
    for stream in listener.incoming()
    {
        //匹配Connection
        match stream
        {
            //Connection无异常
            Ok(stream) =>
            {
                //打印Connection ID
                println!("Connection Success: {}", stream.peer_addr().unwrap());
                //起新线程执行Server端业务
                thread::spawn(move || {handle_client(stream)});
            }
            //Connection异常
            Err(e) =>
            {
                println!("Connection Error: {}", e);
            }
        }
    }
    //析构Server
    drop(listener);
}

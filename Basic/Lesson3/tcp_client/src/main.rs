
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;


fn main() 
{
    //Server端口
    let server_addr = "localhost:3333".to_string();
    //匹配TcpStream连接结果
    match TcpStream::connect(server_addr)
    {
        //Connection正常
        Ok(mut stream) =>
        {
            println!("Connected To Server Success In Port 3333");

            println!("Please Type Something or Press x To Exit:");

            //创建输入缓存
            let mut input_str = String::new();
            input_str.clear();
            //将Client端控台输入写入输入缓存
            std::io::stdin().read_line(&mut input_str).unwrap();
            
            //如果输入不是'x'，循环Request
            while input_str != "x\r\n"
            {
                println!("Client Request: {}", input_str);

                //将输入缓存转为[u8]
                let input_bytes = input_str.as_bytes();

                //将输入缓存写入stream
                stream.write(input_bytes).unwrap();
                
                println!("Request Sending, Waite For Response...\r\n");
                
                //创建Response缓存
                let mut respond_msg = [0 as u8; 128];
                
                //从stream中读取Server的Response，并对读取结果匹配
                match stream.read(&mut respond_msg)
                {
                    //读取无异常
                    Ok(size) =>
                    {
                        //Response信息长度大于0
                        if size > 0
                        {
                            //打印Response缓存
                            let text = from_utf8(&respond_msg).unwrap();
    
                            println!("Server Response: {}", text);
                        }
                        //Response信息为空
                        else
                        {
                            println!("Client Send Nothing");
                        }
    
                    },
                    //读取异常
                    Err(e) =>
                    {
                        println!("Server Response Failed: {}", e);
                    }
                }

                println!("Please Type Something or Press x To Exit:");
                //清理输入缓存并读取下次输入
                input_str.clear();
                std::io::stdin().read_line(&mut input_str).unwrap();
            }
        },
        //Connection异常
        Err(e) =>
        {
            println!("Failed Connect To Server: {}", e);
        }
    }
    println!("Connection Terminated");
}

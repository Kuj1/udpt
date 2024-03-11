use std::i32;

use tokio::net::UdpSocket;
use tokio::time;

use thermometer::thermo_socket::ThermoSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    loop {
        let mut thermo_socket = ThermoSocket::new([127, 0, 0, 1], 6861);
        let data = thermo_socket.get_temp().to_be_bytes();
        let mut buff = [0u8; 4];

        let receiver_socket = UdpSocket::bind("127.0.0.1:6851").await?;
        let sender_socket = thermo_socket.bind_socket().await?;

        send(&sender_socket, "127.0.0.1:6851".to_string(), data).await
            .expect("[ERROR]: With connection or sending data");

        let handled_thread = tokio::spawn(async move { listen(&receiver_socket, &mut buff).await});

        handled_thread.await.unwrap();
    }
}

fn convert_to_arr(b_slice: &[u8]) -> [u8; 4] {
    b_slice
        .try_into()
        .expect("[ERROR]: Slice with incorrect length")
}

async fn send(sender_socket: &UdpSocket, addr: String, data: [u8; 4]) -> Result<usize, std::io::Error> {
    match sender_socket.connect(addr).await {
        Ok(()) => {
            println!(
                "Sended by the {} from the main thread",
                &sender_socket.local_addr().unwrap()
            );
            let send_data = sender_socket
                .send(&data).await
                .expect("[ERROR]: Error while sending data");
            Ok(send_data)
        }
        Err(e) => Err(e),
    }
}

async fn listen(receiver_socket: &UdpSocket, buff: &mut [u8]) {
    match receiver_socket.recv_from(buff).await {
        Ok(received) => {
            let filled_buff = &mut buff[..received.0 as _];
            let array = convert_to_arr(filled_buff);
            let temp = i32::from_be_bytes(array);
            println!(
                "\tReceived by the {} in child thread",
                receiver_socket.local_addr().unwrap()
            );
            println!("\t\tTemperature: {:?}\n", temp);
            time::sleep(time::Duration::from_secs(1)).await;
        }
        Err(e) => println!("recv function failed: {e:?}"),
    }
}
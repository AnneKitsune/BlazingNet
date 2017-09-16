use std::net::UdpSocket;
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

/*pub fn create_socket()->UdpSocket{
	let mut socket = UdpSocket::bind("127.0.0.1:4566").expect("Failed to bind UdpSocket");

    // read from the socket
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf)?;

    // send a reply to the socket we received data from
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src)?;
}

pub fn client(){

}
pub fn server(){

}*/

pub struct ConnectionController{
	owner:Option<NetworkIdentity>,
	clients:Vec<NetworkIdentity>,
	identity:NetworkIdentity,
	frame_send_batches:HashMap<NetworkIdentity,NetworkMessageBatch>,
	supported_protos:Vec<NetworkProtocol>,
}
impl ConnectionController{
	pub fn new()->ConnectionControllerBuilder{
		ConnectionControllerBuilder::new()
	}
	pub fn supports_protocol(&self,proto:NetworkProtocol)->bool{
		self.supported_protos.contains(&proto)
	}
	fn check_unsupported_proto(&self,proto:NetworkProtocol){
		if !supports_protocol(proto){
			panic!("Tried to call ConnectionController using a protocol not supported by it. Did you forget to register it?");
		}
	}
	pub fn send_to(&self,identity:NetworkIdentity,message:NetworkMessage,protocol:NetworkProtocol){

	}
	pub fn send_to_clients(&self,message:NetworkMessage,protocol:NetworkProtocol){
		for c in self.clients{
			self.send_to(c,message,protocol);
		}
	}
	pub fn send_to_all(&self,message:NetworkMessage,protocol:NetworkProtocol){
		send_to_clients(message,protocol);
		send_to_owner(message,protocol);
	}
	pub fn send_to_owner(&self,message:NetworkMessage,protocol:NetworkProtocol){
		if let Some(o) = self.owner{
			self.send_to(self.owner,message,protocol);
		}else{
			panic!("ConnectionController::send_to_owner No owner was defined for this ConnectionController");
		}
	}
	pub fn send_to_all_except(&self,identity:NetworkIdentity,message:NetworkMessage,protocol:NetworkProtocol){
		if let Some(o) = self.owner{
			if o != identity{
				self.send_to(o,message,protocol);
			}
		}
		for c in self.clients{
			if c != identity{
				self.send_to(c,message,protocol);
			}
		}
	}
	pub fn connection_dropped(&mut self,connection:NetworkConnection){
		//If client, drop it
		for c in self.clients{
			if c.connections().contains(connection){
				self.clients.remove(c);
			}
		}
		//If owner, RIP
		if let Some(o) = self.owner{
			if o.connections().contains(connection){
				self.disconnected_from_owner();
			}
		}
	}
	pub fn disconnect_from_owner(&mut self){

		self.disconnected_from_owner();
	}
	pub fn disconnected_from_owner(&mut self){
		//I don't actually know what happens if we lose connection.
		//I guess we should run a closer and let the user decide what his client should do.
	}

	pub fn send_to_all_except_self(&self,message:NetworkMessage,protocol:NetworkProtocol){
		send_to_all_except(self.identity,message,protocol);
	}
	pub fn receive_frame_messages(&self,)->Vec<NetworkMessage>{

	}
}

pub struct ConnectionControllerBuilder{
	owner:Option<NetworkIdentity>,
	supported_protos:Vec<NetworkProtocol>,
}
impl ConnectionControllerBuilder{
	pub fn new()->ConnectionControllerBuilder{
		ConnectionControllerBuilder{
			owner:None,
			supported_protos:Vec::new(),
		}
	}
	pub fn with_owner(mut self,owner:NetworkIdentity)->ConnectionControllerBuilder{
		self.intern.owner = Some(owner);
		self
	}
	pub fn with_protocol(mut self,proto:NetworkProtocol)->ConnectionControllerBuilder{
		if !self.intern.supported_protos.contains(&proto){
			self.intern.supported_protos.push(proto);
		}
		self
	}
	//Should return result
	pub fn as_client_of(mut self,target_ip:IpAddr,target_port:u16)->ConnectionControllerBuilder{
		//try to connect or whatever
		//self.owner = new NetworkConnection
		self
	}
	pub fn build(self)->ConnectionController{
		ConnectionController{
			owner:None,
			clients:Vec::new(),
			identity:NetworkIdentity::new(),
			frame_send_batches:HashMap::new(),
			supported_protos:Vec::new(),
		}
	}
}

pub struct NetworkIdentity{
	connections:HashMap<NetworkProtocol,NetworkConnection>,
}
impl NetworkIdentity{
	pub fn new()->NetworkIdentity{
		NetworkIdentity{
			connections:HashMap::new(),
		}
	}
	pub fn connections(&self)->HashMap<NetworkProtocol,NetworkConnection>{
		self.connections
	}
}

pub struct NetworkConnection{
	proto:NetworkProtocol,
}

pub enum NetworkProtocol{
	HTTP,
	TCP,
	UDP,
}
pub enum Message{
	TestMsg(utctime:u64),
}

pub struct Networked<A>{
	owner:NetworkIdentity,
	element:A,
	uuid:String,
	allow_partial_ownership:bool,
}
impl<A> Networked<A>{
	fn new:
}
use std::collections::HashSet;
use yew::worker::
{
	Agent,
	AgentLink,
	Context,
	HandlerId,
};

use crate::data::water_presence::WaterPresence;

#[derive(Clone)]
pub enum MenuState
{
	Open,
	Collapsed,
}

#[derive(Clone)]
pub enum Request
{
	SetMenuState(MenuState),
	WaterPresence(HashSet<WaterPresence>),
}

pub struct Settings
{
	link: AgentLink<Self>,
	subscribers: HashSet<HandlerId>,
}

impl Agent for Settings
{
	type Reach = Context<Self>;
	type Message = ();
	type Input = Request;
	type Output = Request;

	fn create(link: AgentLink<Self>) -> Self
	{
		Self
		{
			link,
			subscribers: HashSet::new(),
		}
	}

	fn update(&mut self, _: Self::Message)
	{
	}

	fn handle_input(&mut self, message: Self::Input, _: HandlerId)
	{
		for subscriber in &self.subscribers
		{
			self.link.respond(*subscriber, message.clone());
		}
	}

	fn connected(&mut self, id: HandlerId)
	{
		self.subscribers.insert(id);
	}

	fn disconnected(&mut self, id: HandlerId)
	{
		self.subscribers.remove(&id);
	}
}

use std::
{
	cell::RefCell,
	collections::HashSet,
	rc::Rc,
};
use yew::worker::{ Agent, AgentLink, Context, HandlerId };
use crate::agents::filter::FilterView;

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
	FilterChanged(Rc<RefCell<dyn FilterView>>),
}

#[derive(Clone)]
pub enum Response
{
	MenuStateChanged(MenuState),
	FilterChanged(Rc<RefCell<dyn FilterView>>),
}

pub struct Settings
{
	link: AgentLink<Self>,
	subscribers: HashSet<HandlerId>,
}

impl Settings
{
	fn process(&self, message: Request) -> Option<Response>
	{
		match message
		{
			Request::SetMenuState(menu_state) => Some(Response::MenuStateChanged(menu_state)),
			Request::FilterChanged(filter) => Some(Response::FilterChanged(filter)),
		}
	}
}

impl Agent for Settings
{
	type Reach = Context<Self>;
	type Message = ();
	type Input = Request;
	type Output = Response;

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
		if let Some(response) = self.process(message)
		{
			for subscriber in &self.subscribers
			{
				self.link.respond(*subscriber, response.clone());
			}
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

---
title: "You can ask AI about People"
description: "LLMs can also be interpreted as databases, which you can query using natural language."
categories: [ai, llm, data, privacy, security]
icon: rust
date: 2025-08-19
preview_image_url: "https://images.unsplash.com/photo-1506322526487-d8a94af63688?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&w=640"
published: true
---

This weekend while sharing some time with some folks at a bar in the city, we were discussing
GPT-5 model release which came up recently (August 7, 2025), and all the benefits around it.

As part of the discussion, we were talking about how LLMs can be used to query data, and how
you could use it to retrieve certain examples of data, of course much of the data is _hashed_ or
_anonymized_, but given that LLM Chats today comes with AI Agents at their disposition, you can
ask the AI Agent to retrieve data about people on the internet, and it might also use part of the
_public_ flagged data to retrieve information about these people.

So I tried with myself, previously I used to present on the internet as "Esteban Borai", so I assumed
most of the data in their dataset would be related to that name, so I asked the AI Agent
"Who is Esteban Borai?" and after some seconds with the _Searching the web..._ loader I saw my own summary.

![AI Agent Summary on Esteban Borai](/notes/images/018-chat-response-01.png)

How cool is it?

Then I tried with my current domain/username "Leo Borai", and I got a similar result, but this time it was more focused on my
current work, and it also included some of my social media accounts.

![AI Agent Summary on Leo Borai](/notes/images/018-chat-response-02.png)

## Conclusion

Data is not actually comming from the train data in the LLM, but rather from the web search
results, which is why it is so accurate, and it also shows how LLMs can be interpreted as databases,
which you can query using natural language.

This works pretty much the same on every other topic, perhaps more common surnames might not be
as accurate, or if there happen to be a famous "Esteban/Leo Borai", but it kinda feels like
the AI Agent is some kind of private detective, doesn't it?

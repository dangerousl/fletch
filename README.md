# fletch
and/or fern

All of this is TBD, nothing is here yet.

## What is it
Fletch is meant to be a fast, scalable, modern reverse/edge/service proxy that can handle the requirements of a medium or large enterprise without requiring an entire engineering team to configure and maintain.
It specializes in requests and interactions or data flow that occurs between the internal network (data center, private cloud, etc.) and the external network (the public internet).

It is meant to be easily configurable with json or yaml and easily understood by all audiences of engineers from all levels of experience.

It is also flexible by design, allowing a custom filter chain and rulesets per entry configured.

## Why would I want to use it
Do existing options seem too limiting or not fitting the problem statement very well?
Are you tired of needing to get specialized help to configure a reverse proxy?

You can avoid this (someday) by using Fletch instead. (unless it gets named Fern instead)

## How is it going to work
Fletch operates with the idea of a variety of handlers that can interact with an incoming http request.
These handlers are meant to cover both generic use-cases and be extensible to more specific requirements where possible.

Fletch can be configured with entries to match your services by path or header and route. Requests to your defined backend will pass through a filter chain that is defined custom for each entry.
## Where can it be deployed
Fletch can be deployed as far out to the edge as you like.
Or, fletch could be deployed as a sidecar configuration.

Ideally anywhere you would accept north-south traffic from a L4 load balancer, GTM. or similar. 
Additionally, some functionality of Fletch may be available to deploy within wasm module workflows.

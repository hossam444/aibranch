import { openai } from "@ai-sdk/openai";
import VoltAgent, { Agent } from "@voltagent/core";
import { createPinoLogger } from "@voltagent/logger";
import honoServer from "@voltagent/server-hono";

const logger = createPinoLogger({
	name: "aibranch-agent",
	level: "info",
});

const agent = new Agent({
	name: "aibranch-agent",
	instructions: "You are a helpful assistant.",
	model: openai("gpt-5-mini"),
});

new VoltAgent({
	agents: { agent },
	server: honoServer(),
	logger,
});

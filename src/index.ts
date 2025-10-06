import { openai } from "@ai-sdk/openai";
import VoltAgent, { Agent } from "@voltagent/core";
import { createPinoLogger } from "@voltagent/logger";
import serverlessHono from "@voltagent/serverless-hono";
import { branchNameGeneratorWorkflow } from "./workflows/branch-name-generator";

type Env = {
	OPENAI_API_KEY: string;
	VOLTAGENT_PUBLIC_KEY?: string;
	VOLTAGENT_SECRET_KEY?: string;
};

const logger = createPinoLogger({
	name: "aibranch-agent",
	level: "info",
});

const agent = new Agent({
	name: "aibranch-agent",
	instructions: "You are a helpful assistant.",
	model: openai("gpt-5-mini"),
});

const voltAgent = new VoltAgent({
	agents: { agent },
	serverless: serverlessHono(),
	logger,
	workflows: { branchNameGeneratorWorkflow },
});

export default voltAgent.serverless().toCloudflareWorker();

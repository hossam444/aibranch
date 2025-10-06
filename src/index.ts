import { openai } from "@ai-sdk/openai";
import VoltAgent, { Agent, VoltOpsClient } from "@voltagent/core";
import { createPinoLogger } from "@voltagent/logger";
import serverlessHono from "@voltagent/serverless-hono";
import { branchNameGeneratorWorkflow } from "./workflows/branch-name-generator";

type Env = {
	OPENAI_API_KEY: string;
	VOLTAGENT_PUBLIC_KEY?: string;
	VOLTAGENT_SECRET_KEY?: string;
};

const getEnv = (key: keyof Env): string => process.env[key] ?? "";

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
	voltOpsClient: new VoltOpsClient({
		publicKey: getEnv("VOLTAGENT_PUBLIC_KEY"),
		secretKey: getEnv("VOLTAGENT_SECRET_KEY"),
	}),
});

export default voltAgent.serverless().toCloudflareWorker();

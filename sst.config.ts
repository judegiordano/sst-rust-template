import { SSTConfig } from 'sst'
import { Function, type StackContext } from 'sst/constructs'

function ApiStack({ stack }: StackContext) {
	const api = new Function(stack, 'api', {
		handler: 'src/bin/handlers/api.rs',
		url: {
			cors: true
		},
		logRetention: 'one_week'
	})
	stack.addOutputs({
		url: api.url
	})
}

export default {
	config(_input) {
		return {
			name: 'aws-sst-template',
			region: 'us-east-1',
		}
	},
	stacks(app) {
		app.setDefaultFunctionProps({
			runtime: 'rust',
			architecture: 'arm_64',
			environment: {
				STAGE: app.stage,
				REGION: app.region
			}
		})
		app.stack(ApiStack)
	},
} satisfies SSTConfig

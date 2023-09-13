import { SSTConfig } from 'sst'
import { Api, Function, type StackContext } from 'sst/constructs'

function ApiStack({ stack }: StackContext) {
	const api = new Function(stack, 'api', {
		handler: 'handlers/api/main.rs',
		url: {
			cors: true
		},
		logRetention: 'one_week'
	})
	stack.addOutputs({
		endpoint: api.url
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
			}
		})
		app.stack(ApiStack)
	},
} satisfies SSTConfig

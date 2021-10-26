import axios from 'axios';

const url = (path) => new URL(path, 'https://alglobo.herokuapp.com/').href;

export const postRequest = ({ origin, destiny, airline, package: hotel }) =>
	axios
		.post(url('/request'), {
			body: {
				origin,
				destiny,
				airline,
				package: hotel
			}
		})
		.then(({ data }) => data)
		.catch((err) => {
			console.error(err);
			return null;
		});

export const getRequest = async (reqId) =>
	axios
		.get(url('/request'), {
			params: {
				id: reqId
			}
		})
		.then(({ data }) => data)
		.catch((err) => {
			console.error(err);
			return null;
		});

export const postRequests = async (reqs) => {};
export const getRequests = async (reqs) => {};

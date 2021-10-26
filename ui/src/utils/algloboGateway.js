import axios from 'axios';

const url = (path) => new URL(path, 'https://alglobo.herokuapp.com/').href;

export const postRequest = ({ origin, destiny, airline, package: hotel }) =>
	axios
		.post(url('/request'), {
			origin,
			destiny,
			airline,
			package: hotel
		})
		.then(({ data }) => data)
		.catch((err) => {
			const msg = err.response?.data;
			return msg ?? null;
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
			console.log('err:', err);
			return null;
		});

export const postRequests = async (reqs) => {
	const promises = reqs.map((req) => postRequest(req));
	const responses = await Promise.all(promises);

	return responses;
};

export const getRequests = async (reqs) => {};

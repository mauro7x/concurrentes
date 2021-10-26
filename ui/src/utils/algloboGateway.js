import axios from 'axios';

const url = (path) => new URL(path, 'https://alglobo.herokuapp.com/').href;

const postRequest = ({ origin, destiny, airline, package: hotel }) =>
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

const getRequest = async (reqId) =>
	axios
		.get(url('/request'), {
			params: {
				id: reqId
			}
		})
		.then(({ data }) => data)
		.catch((err) => {
			const msg = err.response?.data;
			return msg ?? null;
		});

export const getMetrics = async () =>
	axios
		.get(url('/metrics'))
		.then(({ data }) => JSON.stringify(data, null, 2))
		.catch((err) => {
			const msg = err.response?.data;
			return msg ?? null;
		});

const fnForReq = (fn) => async (reqs) => {
	const promises = reqs.map((req) => fn(req));
	const responses = await Promise.all(promises);

	return responses;
};

export const postRequests = async (reqs) => fnForReq(postRequest)(reqs);
export const getRequests = async (reqs) => fnForReq(getRequest)(reqs);

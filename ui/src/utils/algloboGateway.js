import axios from 'axios';

// const url = (path) => new URL(path, 'https://alglobo.herokuapp.com/').href;
const url = (path) => new URL(path, 'http://localhost:8081/').href;

const postRequest = (req) => {
	console.log('fetching:', url);
	return axios.get(url('/metrics'));
};
const getRequest = (req) => {};

export const postRequests = async (reqs) => postRequest();
export const getRequests = async (reqs) => {};

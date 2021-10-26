import React, { useState } from 'react';
import {  inputPlaceholder, outputPlaceholder, invalidInputMsg, noResponsesMsg } from './messages';
import {
	getMetrics,
	getRequests,
	postRequests
} from '../../utils/algloboGateway';

import { validReqIds, parseInputRequests } from '../../utils/parser';

const RequestPoster = () => {
	const [inputValue, setInputValue] = useState('');
	const [outputValue, setOutputValue] = useState('');
	const [reqIds, setReqIds] = useState([]);
	const [fetching, setFetching] = useState(false)

	const onSend = async () => {
		const reqs = parseInputRequests(inputValue);
		if (!reqs) {
			setOutputValue(invalidInputMsg);
			setFetching(false);
			return;
		}

		setFetching(true);
		const responses = await postRequests(reqs);
		const output = responses.reduce(
			(msg, response) => `${msg}${msg ? '\n' : ''}${response}`,
			''
		);
		const validIds = validReqIds(responses);
		setReqIds(validIds)
		setOutputValue(`[POST /requests] SERVER OUTPUT:\n\n${output}`);
		setFetching(false);
	};
	const onCheckStatus = async () => {
		if (!reqIds.length) {
			setOutputValue(noResponsesMsg)
			setFetching(false);
			return;
		}

		setFetching(true);
		const responses = await getRequests(reqIds);
		const output = responses.reduce(
			(msg, response) => `${msg}${msg ? ',\n' : ''}${JSON.stringify(response, null, 2)}`,
			''
		);
		setOutputValue(`[GET /requests] SERVER OUTPUT:\n\n${output}`);
		setFetching(false);
	};
	const onGetMetrics = async () => {
		setFetching(true);
		const metrics = await getMetrics();
		setOutputValue(`[GET /metrics] SERVER OUTPUT:\n\n${metrics}`);
		setFetching(false);
	}
	const onClean = () => {
		setInputValue('');
		setOutputValue('');
		setReqIds([])
	};

	const handleInputChange = (event) => {
		const { value } = event.target;
		setInputValue(value);
	};

	return (
		<div className='RequestPoster'>
			<textarea
				value={inputValue}
				onChange={handleInputChange}
				rows='10'
				className='input console'
				placeholder={inputPlaceholder}
			/>
			<div className='actions'>
				<button
					disabled={inputValue === '' || fetching}
					className={`up-action ${inputValue === '' || fetching ? 'btn-disabled' : ''}`}
					onClick={() => {
						setFetching(true);
						onSend();
					}}
				>
					Enviar
				</button>
				<button disabled={!reqIds.length || fetching} onClick={() => {
					setFetching(true);
					onCheckStatus();
					}} className={`mid1-action ${!reqIds.length || fetching ? 'btn-disabled' : ''}`}>
					Chequear estado
				</button>
				<button disabled={fetching} onClick={() => {
					setFetching(true);
					onGetMetrics();
					}} className={`mid2-action ${fetching ? 'btn-disabled' : ''}`}>
					Solicitar métricas
				</button>
				<button disabled={fetching} onClick={() => onClean()} className={`down-action ${fetching ? 'btn-disabled' : ''}`}>
					Limpiar
				</button>
			</div>

			<textarea
				rows='10'
				className='output console'
				value={outputValue}
				readOnly
				placeholder={outputPlaceholder}
			/>
		</div>
	);
};

export default RequestPoster;

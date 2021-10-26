import React, { useState } from 'react';
import { inputPlaceholder, outputPlaceholder } from './placeholders';
import {
	getRequest,
	getRequests,
	postRequest,
	postRequests
} from '../../utils/algloboGateway';

import { parseInputRequests } from '../../utils/parser';

const RequestPoster = () => {
	const [inputValue, setInputValue] = useState('');
	const [outputValue, setOutputValue] = useState('');

	const onSend = async () => {
		const reqs = parseInputRequests(inputValue);
		if (!reqs) {
			setOutputValue('Invalid request found');
		}
		const responses = await postRequests(reqs);
		const output = responses.reduce(
			(msg, response) => `${msg}${msg ? '\n' : ''}${response}`,
			''
		);
		setOutputValue(output);
	};
	const onCheckStatus = async () => alert('Check Status');
	const onClean = () => {
		setInputValue('');
		setOutputValue('');
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
					disabled={inputValue === ''}
					className='up-action'
					onClick={() => onSend()}
				>
					Enviar
				</button>
				<button onClick={() => onCheckStatus} className='mid-action'>
					Chequear estado
				</button>
				<button onClick={() => onClean} className='down-action'>
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

import React, { useState } from 'react';
import { inputPlaceholder, outputPlaceholder } from './placeholders';
import { getRequests, postRequests } from '../../utils/algloboGateway';

const RequestPoster = () => {
	const [inputValue, setInputValue] = useState('');
	const [outputValue, setOutputValue] = useState('');

	const onSend = async () => {
		postRequests();
		alert('tenemo response!');
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
				<button className='up-action' onClick={() => onSend()}>
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
				readOnly
				placeholder={outputPlaceholder}
			/>
		</div>
	);
};

export default RequestPoster;

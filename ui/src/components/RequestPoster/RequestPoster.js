import React from 'react';

const inputPlaceholder = `INPUT
Ingrese requests en cada linea utilizando el siguiente formato csv:

origin,destiny,airline,package

Ej:
EZE,JFK,American Airlines,false
EZE,GRU,LATAM,true
`;
const outputPlaceholder = `Output
`;

const RequestPoster = () => {
	return (
		<div className='RequestPoster'>
			<textarea
				rows='10'
				className='input console'
				placeholder={inputPlaceholder}
			/>
			<div className='actions'>
				<button className='up-action'>Enviar</button>
				<button className='mid-action'>Chequear estado</button>
				<button className='down-action'>Limpiar</button>
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

import React from 'react';

const TP1Tab = () => {
	return (
		<div className='Tab'>
			<div className='title'>
				<p className='left'>
					Struct{' '}
					<a href='/' className='link'>
						lib::tp1::AlGlobo
					</a>
				</p>
				<p
					className='right src'
					onClick={() => alert('Tampoco vamos a clonar docs.rs...')}
				>
					[-][src]
				</p>
			</div>
			<p>
				Microservicio que expone una REST API para utilizar el sistema de
				reservas de AlGlobo.
			</p>
		</div>
	);
};

export default TP1Tab;

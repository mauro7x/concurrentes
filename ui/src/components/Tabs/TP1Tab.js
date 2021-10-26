import React from 'react';
import RequestPoster from '../RequestPoster/RequestPoster';

const TP1Tab = () => {
	return (
		<div className='Tab'>
			<div className='title'>
				<p className='left'>
					Struct{' '}
					<a href='#' className='link'>
						lib::tp1::AlGlobo
					</a>
				</p>
				<p className='right src'>[-][src]</p>
			</div>
			<p>
				Microservicio que expone una REST API para utilizar el sistema de
				reservas de AlGlobo.
			</p>
			<RequestPoster />
		</div>
	);
};

export default TP1Tab;

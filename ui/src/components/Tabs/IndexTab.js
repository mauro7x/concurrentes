import React from 'react';

const IndexTab = () => {
	return (
		<div className='Tab'>
			<div className='title'>
				<p className='left'>
					Crate{' '}
					<a href='/' className='link'>
						context_switch
					</a>
				</p>
				<p
					className='right src'
					onClick={() => alert('Tampoco vamos a clonar docs.rs...')}
				>
					[-][src]
				</p>
			</div>
			<p>Grupo conformado por:</p>
			<ul>
				<li>
					Mauro Parafati (
					<a className='link' href='mparafati@fi.uba.ar'>
						mparafati@fi.uba.ar
					</a>
					)
				</li>
				<li>
					Santiago Klein (
					<a className='link' href='sklein@fi.uba.ar'>
						sklein@fi.uba.ar
					</a>
					)
				</li>
				<li>
					Tomás Nocetti (
					<a className='link' href='tnocetti@fi.uba.ar'>
						tnocetti@fi.uba.ar
					</a>
					)
				</li>
			</ul>
			<p>
				para la realización de los trabajos prácticos de la materia{' '}
				<a className='link' href='https://concurrentes-fiuba.github.io/'>
					Técnicas de Programación Concurrente (75.59)
				</a>
				, dictada en la{' '}
				<a className='link' href='https://www.fi.uba.ar/'>
					Facultad de Ingeniería, Universidad de Buenos Aires
				</a>
				, durante el segundo cuatrimestre del 2021.
			</p>
		</div>
	);
};

export default IndexTab;

import React, { useState } from 'react';
import IndexTab from '../Tabs/IndexTab';
import TP1Tab from '../Tabs/TP1Tab';

const Navigation = () => {
	const [activeTab, setActiveTab] = useState('index_tab');

	return (
		<div className='Navigation'>
			<h1 className='title'>use crate::context_switch::*;</h1>
			<h3 className='subtitle'>
				String::from("Técnicas de Programación Concurrente (75.59)")
			</h3>
			<ul className='nav'>
				<li
					className={activeTab === 'index_tab' ? 'active' : ''}
					onClick={() => setActiveTab('index_tab')}
				>
					{'> $ cargo doc'}
				</li>
				<li
					className={activeTab === 'tp1_tab' ? 'active' : ''}
					onClick={() => setActiveTab('tp1_tab')}
				>
					{'> $ cargo run --bin tp1'}
				</li>
			</ul>
			<div className='outlet'>
				{activeTab === 'index_tab' ? (
					<IndexTab />
				) : (
					<React.Fragment></React.Fragment>
				)}

				{activeTab === 'tp1_tab' ? (
					<TP1Tab />
				) : (
					<React.Fragment></React.Fragment>
				)}
			</div>
		</div>
	);
};

export default Navigation;

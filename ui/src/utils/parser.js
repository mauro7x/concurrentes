export const parseInputRequests = (rawInput) => {
	const rows = rawInput.split('\n');
	const reqs = [];
	const invalid = rows.find((row) => {
		const [origin, destiny, airline, hotel, rest] = row.split(',');
		if (
			!origin ||
			!destiny ||
			!airline ||
			!hotel ||
			(hotel !== 'true' && hotel !== 'false') ||
			rest !== undefined
		) {
			return true;
		}

		reqs.push({
			origin,
			destiny,
			airline,
			package: hotel === 'true' ? true : false
		});
		return false;
	});

	if (invalid !== undefined) {
		return null;
	}

	return reqs;
};

export const parseOutput = (rawInput) => {};

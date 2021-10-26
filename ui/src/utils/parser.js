const uuidRegex =
	'^[0-9a-f]{8}-[0-9a-f]{4}-[0-5][0-9a-f]{3}-[089ab][0-9a-f]{3}-[0-9a-f]{12}$';
const validUUID = new RegExp(uuidRegex, 'i');

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

export const validReqIds = (reqIds) =>
	reqIds.filter(
		(reqId) => typeof reqId === 'string' && !!reqId.match(validUUID)
	);

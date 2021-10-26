const csvFormat = `origin,destiny,airline,package`;
const example = `Ej:
EZE,JFK,American Airlines,false
EZE,GRU,LATAM,true`;

// Exports

export const inputPlaceholder = `INPUT
Ingrese requests en cada linea utilizando el siguiente formato csv:

${csvFormat}

${example}`;

export const outputPlaceholder = `OUTPUT
Aquí se mostrarán mensajes y las respuestas del servidor ante las distintas acciones ejecutadas.`;

export const invalidInputMsg = `ERROR: INPUT INVÁLIDO
El input es inválido. Cada linea debe ser una request en el siguiente formato csv:

${csvFormat}

${example}`

export const noResponsesMsg = `ERROR: NO HAY REQUESTS
Para chequear el estado, primero se debe enviar con un input valido un grupo de requests con al menos una.`
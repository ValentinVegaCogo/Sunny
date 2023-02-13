import { print } from './functions.js'
import { NAME, VERSION } from './info.js'
import { create, obj_get_entries, to_string } from './values.js'

function key_throw(value) {
	console.log(`[throwing]`)
	print([value])
	print([])
	print([`${NAME} v${VERSION}`])
	process.exit(1)
}

function key_equals(a, b) {
	if (a.type !== b.type) return false
	//
}

function key_sum(a, b) {
	if (a.type !== b.type) {
		key_throw(`TypeError: cannot sum values of type '${a.type}' and '${b.type}'`)
	}
	if (a.type === 'string') {
		return create.string(a.value + b.value)
	}
	if (a.type === 'array') {
		return create.array(a.value.concat(b.value))
	}
	if (a.type === 'object') {
		const left = obj_get_entries(a)
		const right = obj_get_entries(b)
		console.log(left, right)
		return create.object(left.concat(right))
	}
	key_throw(`TypeError: cannot sum values of type '${a.type}'`)
}

console.log(
	to_string(key_sum(
		create.object([
			['a', create.string('1')]
		]),
		create.object([
			['b', create.string('2')]
		])
	))
)

export {
	key_throw,
	key_equals,
	key_sum
}
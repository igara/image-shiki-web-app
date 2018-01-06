import {m} from '../vendor';
import HeaderComponent from './common/header_component';
import InputFileComponent from './index/input_file_component';

/**
 * Routing URL: //index/
 */
export default class IndexComponent {

	/**
	 * @type {HeaderStore} HeaderStore
	 */
	HeaderStore;

	/**
	 * @constructor
	 * @param {Vnode<A, this>} vnode 
	 */
	constructor(vnode) {
		this.HeaderStore = vnode.HeaderStore;
	}

	/**
	 * Lifecycle: The oninit hook is called before a vnode is touched by the virtual DOM engine.
	 * @param {Vnode<A, this>} vnode 
	 */
	oninit(vnode) {
		this.HeaderStore.header_title_stream('Syonet');
	}

	/**
	 * Lifecycle: Creates a view out of virtual elements.
	 */
	view() {
		return (
<div>
	<HeaderComponent
		HeaderStore={this.HeaderStore}
	/>
	<a href="/analyzeimage" oncreate={m.route.link}>sub</a>
	{this.HeaderStore.s_stream}
	<InputFileComponent />
</div>
		);
	}
}

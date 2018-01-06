// Components
import IndexComponent from '../components/index_component';
import AnalyzeImageComponent from '../components/analyze_image_component';

// Stores
import HeaderStore from '../stores/common/header_store';

const routes = {
	'/': new IndexComponent({
		HeaderStore,
	}),
	'/analyzeimage': new AnalyzeImageComponent({
		HeaderStore,
	}),    
}

export default routes;

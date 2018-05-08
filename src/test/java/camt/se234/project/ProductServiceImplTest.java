package camt.se234.project;

import camt.se234.project.dao.ProductDao;
import camt.se234.project.entity.Product;
import camt.se234.project.service.ProductServiceImpl;
import org.junit.Before;
import org.junit.Test;
import java.util.ArrayList;
import java.util.List;


import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.hasItems;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

public class ProductServiceImplTest {
    ProductDao productDao;
    ProductServiceImpl productService;

    @Before
    public void setup(){
        productDao = mock(ProductDao.class);
        productService = new ProductServiceImpl();
        productService.setProductDao(productDao);
    }
    @Test
    public void TestGetAllProduct(){
        List<Product> mockProduct = new ArrayList<>();
        mockProduct.add(new Product(null,"p0001","test1","test1","test1.jpg",5000));
        mockProduct.add(new Product(null,"p0002","test2","test2","test2.jpg",200));
        mockProduct.add(new Product(null,"p0003","test3","test3","test3.jpg",0));
        mockProduct.add(new Product(null,"p0004","test4","test4","test4.jpg",-50));

        when(productDao.getProducts()).thenReturn(mockProduct);

        assertThat(productService.getAllProducts(),is(mockProduct));
    }
    @Test
    public void TestGetAvailableProduct(){
        List<Product> mockProduct = new ArrayList<>();
        mockProduct.add(new Product(null,"p0001","test1","test1","test1.jpg",5000));
        mockProduct.add(new Product(null,"p0002","test2","test2","test2.jpg",200));
        mockProduct.add(new Product(null,"p0003","test3","test3","test3.jpg",0));
        mockProduct.add(new Product(null,"p0004","test4","test4","test4.jpg",-50));

        when(productDao.getProducts()).thenReturn(mockProduct);


        assertThat(productService.getAvailableProducts(),hasItems(new Product(null,"p0001","test1","test1","test1.jpg",5000),
                new Product(null,"p0002","test2","test2","test2.jpg",200)));
    }


}

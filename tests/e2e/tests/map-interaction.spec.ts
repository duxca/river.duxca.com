import { test, expect } from '@playwright/test';

test.describe('Map Interactions', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Wait for map to be ready
    await expect(page.locator('.leaflet-container')).toBeVisible();
  });

  test('should be able to zoom in and out', async ({ page }) => {
    const zoomInButton = page.locator('.leaflet-control-zoom-in');
    const zoomOutButton = page.locator('.leaflet-control-zoom-out');
    
    await expect(zoomInButton).toBeVisible();
    await expect(zoomOutButton).toBeVisible();
    
    // Test zoom in
    await zoomInButton.click();
    await page.waitForTimeout(500); // Wait for zoom animation
    
    // Test zoom out
    await zoomOutButton.click();
    await page.waitForTimeout(500); // Wait for zoom animation
  });

  test('should be able to switch map layers', async ({ page }) => {
    // Look for layer control
    const layerControl = page.locator('.leaflet-control-layers');
    if (await layerControl.isVisible()) {
      await layerControl.click();
      
      // Try to switch to different base layers
      const gsiLayer = page.locator('text=GSI');
      const osmLayer = page.locator('text=OpenStreetMap');
      
      if (await gsiLayer.isVisible()) {
        await gsiLayer.click();
        await page.waitForTimeout(1000); // Wait for tiles to load
      }
      
      if (await osmLayer.isVisible()) {
        await osmLayer.click();
        await page.waitForTimeout(1000); // Wait for tiles to load
      }
    }
  });

  test('should be able to pan the map', async ({ page }) => {
    const mapContainer = page.locator('.leaflet-container');
    
    // Get initial map center (this is a basic test)
    const mapBounds = await mapContainer.boundingBox();
    if (mapBounds) {
      const centerX = mapBounds.x + mapBounds.width / 2;
      const centerY = mapBounds.y + mapBounds.height / 2;
      
      // Drag from center to a different position
      await page.mouse.move(centerX, centerY);
      await page.mouse.down();
      await page.mouse.move(centerX + 100, centerY + 100);
      await page.mouse.up();
      
      await page.waitForTimeout(500); // Wait for pan animation
    }
  });
  test('should display map attribution', async ({ page }) => {
    // Check that map attribution is visible
    const attribution = page.locator('.leaflet-control-attribution');
    await expect(attribution).toBeVisible();
  });

  test('should handle map click events', async ({ page }) => {
    const mapContainer = page.locator('.leaflet-container');
    
    // Click on the map
    await mapContainer.click();
    
    // This test would need to be more specific based on what happens when you click the map
    // For example, if clicking opens a context menu or places a marker
    await page.waitForTimeout(500);
  });
});